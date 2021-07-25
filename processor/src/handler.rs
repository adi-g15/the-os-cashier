use crypto::sha2::Sha512;
use crypto::digest::Digest; // Read comment below
/*
 *error[E0599]: no method named `input_str` found for struct `Sha512` in the current scope
  --> src/handler.rs:23:13
   |
23 |         sha.input_str("os-cashier");
   |             ^^^^^^^^^ method not found in `Sha512`
   | 
  ::: /home/ag15035/.cargo/registry/src/github.com-1ecc6299db9ec823/rust-crypto-0.2.36/src/digest.rs:66:8
   |
66 |     fn input_str(&mut self, input: &str) {
   |        --------- the method is available for `Sha512` here
   |
   = help: items from traits can only be used if the trait is in scope
   = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
           `use crypto::digest::Digest;`
 * */

use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::{ApplyError, TransactionContext, TransactionHandler};

pub struct OSCashierHandler {
   family_name: String,
   family_versions: Vec<String>,
   namespaces: Vec<String>
}

impl OSCashierHandler {
    pub fn new() -> OSCashierHandler {
        OSCashierHandler {
            family_name: "os-cashier".to_string(),
            family_versions: vec!["0.1".to_string()],
            namespaces: vec![OSCashierHandler::get_prefix()]
        }
    }

    fn get_prefix() -> String {
        let mut sha = Sha512::new();
        sha.input_str("os-cashier");
        sha.result_str()[..6].to_string()   // return first 6 chars in the string
    }
}

impl TransactionHandler for OSCashierHandler {
    fn family_name(&self) -> String {
        self.family_name.clone()    // clone before returning, or else the ownership will transfer
    }

    fn family_versions(&self) -> Vec<String> {
        self.family_versions.clone()    // TODO: Find out whether cloning vector will also clone the entries within it ?
    }

    fn namespaces(&self) -> Vec<String> {
        self.namespaces.clone()
    }

    fn apply(
        &self,
        _request: &TpProcessRequest,
        _context: &mut dyn TransactionContext    // TODO: Read about dyn
        ) -> Result<(), ApplyError>
    {
        return Err(ApplyError::InvalidTransaction("WIP: ABHI COMPLETE NAHI HUA HAI !".to_string()));
    }
}

