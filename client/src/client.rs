use std::collections::BTreeMap;

use std::{fs, path};

use crate::payload::OSCashierPayload;
use protobuf::Message;
use rand::{thread_rng, RngCore};
use sawtooth_sdk::messages::batch::{Batch, BatchHeader, BatchList};
use sawtooth_sdk::messages::transaction::{Transaction, TransactionHeader};
use sawtooth_sdk::signing::{
    self, secp256k1::Secp256k1PrivateKey
};

use crate::payload::Actions;

const FAMILY_NAME: &str = "os-cashier";
const FAMILY_VERSION: &str = "0.1";

pub struct OSCashierClient {
    privatekey: Secp256k1PrivateKey, // read more on 'a
    module_performance: BTreeMap<String, f32>,
    rest_api_url: String,
}

impl OSCashierClient {
    pub fn new(rest_api_url: String) -> OSCashierClient {
        let mut module_performance = BTreeMap::new();

        module_performance.insert("slab_allocator".to_string(), 0.4);
        module_performance.insert("slub_allocator".to_string(), -0.1);
        module_performance.insert("slob_allocator".to_string(), -0.5);
        module_performance.insert("buddy_allocator".to_string(), 0.2);

        /*
         * Getting keyfile as in https://github.com/hyperledger/sawtooth-sdk-python/blob/9ce6d0be599ea89c987da983ebe1c2beac14e6ee/examples/intkey_python/sawtooth_intkey/client_cli/intkey_cli.py#L315
         */
        let current_user = whoami::username();
        let home_dir = match dirs::home_dir() {
            Some(home_dir) => home_dir,
            None => {
                println!("Warning: Couldn't get the home directory path ! OS may not be supported... ");
                println!("Warning: May use random keys for this run...");
                path::PathBuf::new()
            }
        };

        let keys_dir = home_dir.join(".sawtooth").join("keys");
        let keyfile = format!("{}/{}.priv", keys_dir.to_str().unwrap_or("."), current_user);

        let privatekey: Secp256k1PrivateKey;
        if std::path::Path::new(&keyfile).exists() {  // if available, will use keys generated by "sawtooth keygen"
            privatekey = Secp256k1PrivateKey::from_hex(
                fs::read_to_string(&keyfile)
                    .expect("Something went wrong reading the file")
                    .trim(),
            )
            .expect("Couldn't create PrivateKey object using contents of the .priv file");
        } else {
            privatekey = Secp256k1PrivateKey::from_hex(
                &signing::create_context("secp256k1")
                    .expect("ERROR: Couldn't create SECP256k1 context")
                    .new_random_private_key()
                    .expect("Error generating a random key")
                    .as_hex(),
            )
            .expect("Couldn't create PrivateKey object from a random key");
        }

        OSCashierClient {
            rest_api_url,
            privatekey,
            module_performance,
        }
    }

    /*
        Signing -

        let context = create_context("secp256k1").expect("Error creating the right context");
        let crypto_factory = CryptoFactory::new(context.as_ref());

        let signer = crypto_factory.new_signer(private_key.as_ref());
    */

    fn create_transaction(&self, payload_bytes: Vec<u8>, asset_keys: Option<Vec<&str>>) -> Transaction {  // asset_key is used to get asset address
        // Create Header -> Prerequisits: nonce, public key, inputs/outputs, payload_sha512hash
        let nonce = hex::encode( OSCashierClient::get_nonce() );

        let addresses = asset_keys.map(|keys| keys.iter().map(|asset_name| -> String { self.get_address(asset_name) }).collect());

        let inputs_vec = match addresses {
            Some(addresses) => addresses,
            None => vec![]
        };
        let outputs_vec = inputs_vec.clone();

        let mut header = TransactionHeader::new();
        header.set_family_name(FAMILY_NAME.to_string());
        header.set_family_version(FAMILY_VERSION.to_string());
        header.set_nonce(nonce);
        header.set_signer_public_key(self.get_public_key());
        header.set_batcher_public_key(self.get_public_key());
        header.set_inputs(protobuf::RepeatedField::from_vec(inputs_vec));
        header.set_outputs(protobuf::RepeatedField::from_vec(outputs_vec));
        header.set_payload_sha512( hex::encode( openssl::sha::sha512(&payload_bytes).to_vec() ) );

        /* NOTE: hash of bytes is just hex::encode(sha::sha512() ) hash string, though
         *       signature/signed bytes is signer.sign(bytes).as_hex()... there's a difference between these :)
         */

        // Create transaction -> Prerequisits: header_bytes, header_signature, payload_bytes
        let header_bytes = header.write_to_bytes().expect("Error: Couldn't serialise TransactionHeader");
        let header_signature = self.sign_bytes(&header_bytes);

        let mut transaction = Transaction::new();
        transaction.set_header( header_bytes );
        transaction.set_header_signature( header_signature );
        transaction.set_payload( payload_bytes.to_vec() );

        #[cfg(debug_assertions)] {
            println!(
                "TxnHeader: {:?}\n\nTransaction: {:?}\n\n",
                header, transaction
            );
        }

        transaction
    }

    fn create_batch(&self, transactions: Vec<Transaction>) -> Batch {

        /* From Docs ->
         * Once the TransactionHeader is constructed, its bytes are then used to create a signature.
         * This header signature also acts as the ID of the transaction
         */

        // Creating BatchHeader: Prereqs -> public key, transaction ids
        let transaction_ids = transactions.iter()
                                             .map(|trx| trx.get_header_signature().to_string() )
                                             .collect();

        let mut batch_header = BatchHeader::new();
        batch_header.set_signer_public_key(self.get_public_key());
        batch_header.set_transaction_ids( protobuf::RepeatedField::from_vec(transaction_ids) );

        // Creating Batch: Prereqs -> header_bytes, signature, transactions
        let batch_header_bytes = batch_header.write_to_bytes().expect("Error: Couldn't serialize BatchHeader");
        let batch_header_signature = self.sign_bytes(&batch_header_bytes);

        let mut batch = Batch::new();
        batch.set_header(batch_header_bytes);
        batch.set_header_signature(batch_header_signature);
        batch.set_transactions( protobuf::RepeatedField::from_vec(transactions) );

        #[cfg(debug_assertions)] {
            println!(
                "BatchHeader: {:?}\n\nBatches: {:?}\n\n",
                batch_header, batch
            );
        }

        batch
    }

    fn create_batchlist(&self, batches: Vec<Batch>) -> BatchList {
        // Prereqs: batches
        let mut batch_list = BatchList::new();
        batch_list.set_batches( protobuf::RepeatedField::from_vec(batches) );

        #[cfg(debug_assertions)] {
            println!("BatchList: {:?}\n\n", batch_list);
        }

        batch_list
    }

    fn send_transaction(&self, batch_list_bytes: &[u8]) -> Result<String, reqwest::Error> {
        /* If this is a debug build, will write this data to a file too */
        if cfg!(debug_assertions) {
            use std::io::Write;

            println!("[DEBUG BUILD] Writing the bytes to os-cashier.tmp.batches");
            let mut file = std::fs::File::create("os-cashier.tmp.batches").expect("Error creating file");
            match file.write_all(batch_list_bytes) {
                Ok(_ok) => {},
                Err(e) => { println!("Error: {:?}", e) }
            };
        }

        // Using a blocking client... I don't know currently the async await in Rust, may change later
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{}/batches", self.rest_api_url))
            .header("Content-Type", "application/octet-stream")
            .body(batch_list_bytes.to_vec()) // [LEARNT] - static lifetime was required, can also be simply fixed by passing a copy of the slice, as a vector
            .send()?.text();

        match response {
            Ok(res) => {
                println!("{:#?}", res);
                Ok(res)
            },
            Err(e) => {
                println!("Error: {:?}", e);
                Err(e)
            }
        }
    }

    fn get_address(&self, name: &str) -> String {
        let prefix = &hex::encode( openssl::sha::sha512(FAMILY_NAME.as_bytes() ))[0..6];
        let name_hash = &hex::encode( openssl::sha::sha512(name.as_bytes()) )[64..];

        println!("Prefix is: {}", prefix);
        println!("Hash for name: {} is {}, length: {}", name, name_hash, name_hash.len());

        prefix.to_string() + name_hash      // `String + &str` works fine !
    }

    fn get_nonce() -> [u8; 16] {
        // 16 bytes (128 bit) nonce
        let mut nonce = [0u8; 16];
        thread_rng().fill_bytes(&mut nonce);
        nonce
    }

    fn sign_bytes(&self, bytes: &[u8]) -> String {
        let context =
            signing::create_context("secp256k1").expect("Error Creating SECP256k1 Context");
        let crypto_factory = signing::CryptoFactory::new(context.as_ref());

        crypto_factory
            .new_signer(&self.privatekey)
            .sign(bytes)
            .expect("FATAL ERROR: Couldn't Sign Message")
    }

    fn get_public_key(&self) -> String {
        let context =
            signing::create_context("secp256k1").expect("Error Creating SECP256k1 Context");
        let crypto_factory = signing::CryptoFactory::new(context.as_ref());

        crypto_factory
            .new_signer(&self.privatekey)
            .get_public_key()
            .expect("FATAL ERROR: Couldn't get Public Key")
            .as_hex()
    }

    pub fn reg(&self, username: String) {
        let payload_bytes = OSCashierPayload::new(Actions::Register, username.clone()).to_bytes();

        let transaction = self.create_transaction(payload_bytes, Some(vec![&username]));
        let batch       = self.create_batch(vec![transaction]);
        let batch_list  = self.create_batchlist(vec![batch]);

        let batch_list_bytes = batch_list
            .write_to_bytes()
            .expect("Error: Couldn't serialise batch list");

        self.send_transaction(&batch_list_bytes);
    }

    pub fn plug(&self, username: String, module_name: String) {
        let mut payload = OSCashierPayload::new(Actions::PlugMod, username.clone());
        payload.set_module(module_name);

        let payload_bytes = payload.to_bytes();

        self.send_transaction(
            &self.create_batchlist(
                vec![self.create_batch(
                    vec![self.create_transaction(payload_bytes, Some(vec![&username]))]
                )]
            )
            .write_to_bytes()
            .expect("Error: Couldn't serialise batchlist")
        );
    }

    pub fn unplug(&self, username: String, module_name: String) {
        let mut payload = OSCashierPayload::new(Actions::UnplugMod, username.clone());
        payload.set_module(module_name);

        let payload_bytes = payload.to_bytes();

        self.send_transaction(
            &self.create_batchlist(
                vec![self.create_batch(
                    vec![self.create_transaction(payload_bytes, Some(vec![&username]))]
                )]
            )
            .write_to_bytes()
            .expect("Error: Couldn't serialise batchlist")
        );
    }

    pub fn transfer(&self, sender: String, receiver: String, amount: u32) {
        let mut payload = OSCashierPayload::new(Actions::Transfer, sender.clone());
        payload.set_receiver(receiver.clone());
        payload.set_amount(amount);

        let payload_bytes = payload.to_bytes();

        self.send_transaction(
            &self.create_batchlist(
                vec![self.create_batch(
                    vec![self.create_transaction(payload_bytes, Some(vec![&sender,&receiver]))]
                )]
            )
            .write_to_bytes()
            .expect("Error: Couldn't serialise batchlist")
        );
    }

    // pub fn list(&self, _list_modules: bool) {}
    pub fn list_modules(&self) {
        println!("Module -> Performance Benefit\n");
        self.module_performance.iter().for_each(|m| println!("{} -> {}", m.0, m.1));
    }
}
