#![allow(unused)]
use std::collections::BTreeMap;

use std::{fs, path};

use crate::payload::OSCashierPayload;
use sawtooth_sdk::signing::{
    self, secp256k1::Secp256k1PrivateKey, CryptoFactory, PrivateKey, Signer
};
use sawtooth_sdk::messages::transaction::TransactionHeader;
use rand::{RngCore,thread_rng};
use protobuf::Message;

// TODO: Create transactions, according to https://github.com/saan099/sawtooth-test/blob/master/client/index.js

const FAMILY_NAME: &str = "os-cashier";
const FAMILY_VERSION: &str = "0.1";

mod util {
    pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
        hex::encode(bytes)
    }
}

pub struct OSCashierClient {
    rest_api_url: String,
    privatekey: Secp256k1PrivateKey, // read more on 'a
    module_performance: BTreeMap<String, f32>,
}

impl OSCashierClient {
    const INITIAL_POINTS: u32 = 10;
    pub fn new(url: &str) -> OSCashierClient {
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
                println!("Sorry OS maynot be supported... Couldn't get the home directory path !");
                println!("[WARNING] Looking for the keys in current directory");
                path::PathBuf::new()
            }
        };

        let keys_dir = home_dir.join(".sawtooth").join("keys");
        let keyfile = format!("{}/{}.priv", keys_dir.to_str().unwrap_or("."), current_user);

        let privatekey = Secp256k1PrivateKey::from_hex(
            fs::read_to_string(&keyfile)
                .expect("Something went wrong reading the file")
                .trim()
        )
        .expect("Couldn't create PrivateKey object using contents of the .priv file");

        println!("Private Key: {:?}", privatekey.as_hex());
        
        OSCashierClient {
            rest_api_url: url.to_string(),
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

    fn send_rest_api_call() {}

    fn get_nonce() -> [u8; 16] {       // 16 bytes (128 bit) nonce
        let mut nonce = [0u8; 16];
        thread_rng().fill_bytes(&mut nonce);
        nonce
    }

    fn get_public_key(&self) -> String {
        let context = signing::create_context("secp256k1").expect("Error Creating SECP256k1 Context");
        let crypto_factory = signing::CryptoFactory::new(context.as_ref());

        crypto_factory.new_signer(&self.privatekey).get_public_key().expect("FATAL ERROR: Couldn't get Public Key").as_hex()
    }

    pub fn reg(&self, username: &str) {
        // Step 1: Create Payload
        let payload_bytes = OSCashierPayload {
            name: username.to_string(),
            curr_mods: vec![],
            points: OSCashierClient::INITIAL_POINTS,
        }
        .to_bytes();

        // Step 2: Create Transaction
        //      2.1: Transaction Header
        //      2.2: Transaction
        let nonce = OSCashierClient::get_nonce();

        let mut header = TransactionHeader::new();
        header.set_family_name(FAMILY_NAME.to_string());
        header.set_family_version(FAMILY_VERSION.to_string());
        header.set_nonce( util::bytes_to_hex_string(&nonce) );

        header.set_inputs( protobuf::RepeatedField::from(
            vec![String::from(
                "1cf1266e282c41be5e4254d8820772c5518a2c5a8c0c7f7eda19594a7eb539453e1ed7",
            )]
        ));
        header.set_outputs( protobuf::RepeatedField::from(
            vec![String::from(
                "1cf1266e282c41be5e4254d8820772c5518a2c5a8c0c7f7eda19594a7eb539453e1ed7",
            )]
        ));
        header.set_signer_public_key(self.get_public_key());
        header.set_batcher_public_key(self.get_public_key());

        header.set_payload_sha512(
            util::bytes_to_hex_string( &openssl::sha::sha512( &payload_bytes ).to_vec() )
        );

        // Now, transaction header object done, serialise header now
        let header_bytes = header.write_to_bytes().expect("Couldn't Serialise TransactionHeader");

        

    }

    pub fn list(&self, list_modules: bool) {}

    pub fn plug(&self, username: &str, module_name: &str) {}

    pub fn unplug(&self, username: &str, module_name: &str) {}
}
