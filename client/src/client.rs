use std::collections::BTreeMap;

use whoami;
use std::path;

// TODO: Create transactions, according to https://github.com/saan099/sawtooth-test/blob/master/client/index.js

pub struct OSCashierClient {
    rest_api_url: String,
    keyfile: String,
    module_performance: BTreeMap<String,f32>
}

impl OSCashierClient {
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

        /*
         * TODO: We need a 'signer', instead of this keyfile path actually,... See this code in python:
         * 
         * https://github.com/hyperledger/sawtooth-sdk-python/blob/9ce6d0be599ea89c987da983ebe1c2beac14e6ee/examples/intkey_python/sawtooth_intkey/client_cli/intkey_client.py#L46-L62
         * */

        OSCashierClient {
            rest_api_url: url.to_string(),
            keyfile: keyfile,
            module_performance: module_performance
        }
    }

    fn send_rest_api_call() {

    }

    pub fn reg(&self, username: &str) {
        // {user: username} is the payload
    }

    pub fn list(&self, list_modules: bool) {

    }

    pub fn plug(&self, username: &str, module_name: &str) {

    }

    pub fn unplug(&self, username: &str, module_name: &str) {

    }
}
