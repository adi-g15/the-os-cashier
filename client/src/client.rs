use std::collections::BTreeMap;

use whoami;
use std::{fs,path};

use sawtooth_sdk::signing::{Signer, create_context, CryptoFactory};
use crate::payload::OSCashierPayload;

// TODO: Create transactions, according to https://github.com/saan099/sawtooth-test/blob/master/client/index.js

pub struct OSCashierClient<'a> {
    rest_api_url: String,
    signer: Signer<'a>, // read more on 'a
    module_performance: BTreeMap<String,f32>,
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

        let privatekey = fs::read_to_string(&keyfile)
            .expect("Something went wrong reading the file");

        println!("Private Key: {}", privatekey);

        let context = create_context("secp256k1").expect("Couldn't create sec256k1 context !!");
        let crypto_factory = CryptoFactory::new(&context);
        let signer = crypto_factory.new_signer(&privatekey);

        OSCashierClient {
            rest_api_url: url.to_string(),
            signer: signer,
            module_performance: module_performance
        }
    }

    fn send_rest_api_call() {

    }

    pub fn reg(&self, username: &str) {
        let payload = OSCashierPayload::new(username, [], OSCashierClient::INITIAL_POINTS);
    }

    pub fn list(&self, list_modules: bool) {

    }

    pub fn plug(&self, username: &str, module_name: &str) {

    }

    pub fn unplug(&self, username: &str, module_name: &str) {

    }
}
