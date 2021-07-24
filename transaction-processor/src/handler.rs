use crypto::sha2::Sha512;

use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::{ApplyError};

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
            namespaces: vec![get_prefix()]
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
        request: &TpProcessRequest,
        context: &mut dyn TransactionContext    // TODO: Read about dyn
        ) -> Result<(), ApplyError>
    {
        return ApplyError::InvalidTransaction("WIP: ABHI COMPLETE NAHI HUA HAI !".to_string());
    }
}

