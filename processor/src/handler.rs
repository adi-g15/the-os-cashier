use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::{ApplyError, TransactionContext, TransactionHandler};

use crate::payload_impl::OSCashierPayload;

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
        _context: &mut dyn TransactionContext    // TODO: Read about dyn
        ) -> Result<(), ApplyError>
    {
        let header = &request.header;
        let _public_key = match header.as_ref() {
            Some(h) => &h.signer_public_key,
            None => {
                return Err(ApplyError::InvalidTransaction(
                    "Invalid Header".to_string()
                ))
            }
        };

        let payload = OSCashierPayload::from_bytes( &request.payload );

        println!("Received Payload:\n{:#?}", payload);

        // let context = OSCashierState::new( context );

        /*
            An example request object:

            Request: {
                header {
                    batcher_public_key: "0310a004db17da09c2bbb477cd39fa9701873b14a173d3d167c8ac20ebd336ef9d"
                    family_name: "os-cashier"
                    family_version: "0.1"
                    inputs: "3163cafd9db05ee8b325c0ad36438b43fec8510c204fc1c1edb21d0941c00e9e2c1ce2"
                    nonce: "7556ebfbb3bbfaa40f731ee14b22c05a"
                    outputs: "3163cafd9db05ee8b325c0ad36438b43fec8510c204fc1c1edb21d0941c00e9e2c1ce2"
                    payload_sha512: "f74b07cefa9585b3b1f99545c5790cf3ebefdbf22079a5e8b03226a4110fc7a6838dfa02af2c39c216926b40e83f773f5b82e624b215f0b6d15a48c5f480426b"
                    signer_public_key: "0310a004db17da09c2bbb477cd39fa9701873b14a173d3d167c8ac20ebd336ef9d"
                }
                payload: "\243dnamedusericurr_mods\200fpoints\n"
                signature: "757438c4412ef9ab0f3e517acc914ab3076c2d153711623d6dba459a9458d329365bddaeddf233941b2f38d9c9d6710d45ea7e659a65b71273d86c8c24315f65"
                context_id: "b1ad2f37718e47f7bdd4cff5278bbe29"
            }

            Context.get_state_entries([]): Ok([])
         * */
        #[cfg(debug_assertions)] {
            println!("Request: {:#?}", request);
        }

        /*
         * Errors: ApplyError or ContextError
         * 
         * ApplyError: Either InvalidTransaction or InternalError... In invalid transaction, it will retry again once each second, or slightly faster, and will keep retry even if the tp unregisters then registers again.
         *                                                           In internal error, it retries EACH MILLISECONDS, don't return that, agar logs padhne layak chahiye to !
        */
        Err(ApplyError::InvalidTransaction("WIP: ABHI COMPLETE NAHI HUA HAI !".to_string()))
    }
}
