use std::collections::BTreeMap;

use std::{fs, path};
use whoami;

use crate::payload::OSCashierPayload;
use sawtooth_sdk::signing::{
    create_context, secp256k1::Secp256k1PrivateKey, CryptoFactory, PrivateKey, Signer,
};

// TODO: Create transactions, according to https://github.com/saan099/sawtooth-test/blob/master/client/index.js

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
                .as_str(),
        )
        .expect("Couldn't create PrivateKey object using contents of the .priv file");

        println!("Private Key: {:?}", privatekey.as_hex());

        OSCashierClient {
            rest_api_url: url.to_string(),
            privatekey: privatekey,
            module_performance: module_performance,
        }
    }

    pub fn signer(&self) -> Signer {
        let context = create_context("secp256k1")
        .expect("Couldn't create sec256k1 context !!");
    
        CryptoFactory::new(
                context.as_ref()
        )
        .new_signer(&self.privatekey)
    }

    fn send_rest_api_call() {}

    pub fn reg(&self, username: &str) {
        let payload_bytes = OSCashierPayload::new(
            username.to_string(),
            vec![],
            OSCashierClient::INITIAL_POINTS,
        )
        .to_bytes();
    }

    pub fn list(&self, list_modules: bool) {}

    pub fn plug(&self, username: &str, module_name: &str) {}

    pub fn unplug(&self, username: &str, module_name: &str) {}
}
