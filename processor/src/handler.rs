use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::{ApplyError, TransactionContext, TransactionHandler};

pub struct OSCashierHandler {
   family_name: String,
   family_versions: Vec<String>,
   namespaces: Vec<String>
}

impl OSCashierHandler {
    // const FAMILY_NAME: String = "os-cashier".to_string();
    // const FAMILY_VERSIONS: Vec<String> = vec!["0.1".to_string()];
    pub fn new() -> OSCashierHandler {
        OSCashierHandler {
            family_name: "os-cashier".to_string(),
            family_versions: vec!["0.1".to_string()],
            namespaces: vec![OSCashierHandler::get_prefix()]
        }
    }

    fn get_prefix() -> String {
        hex::encode(
            openssl::sha::sha512("os-cashier".as_bytes())
        )[0..6].to_string() // return first 6 chars in the string
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
        let header = &request.header;

        #[cfg(debug_assertions)] {
            println!("Headers: {:?}\n\n\n", header);
            println!("Request:\n\n{:?}\n\n\n\n\n", request);
            println!("Context: {:?}", context.get_state_entries(&[]));
        }

        return Err(ApplyError::InvalidTransaction("WIP: ABHI COMPLETE NAHI HUA HAI !".to_string()));
    }
}

