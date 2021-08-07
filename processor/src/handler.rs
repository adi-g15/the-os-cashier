use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::{ApplyError, TransactionContext, TransactionHandler};

use crate::payload_impl::{OSCashierPayload,Actions};
use crate::structs::state::{OSCashierState,_InternalOSCashierState};

pub struct OSCashierHandler {
   family_name: String,
   family_versions: Vec<String>,
   namespaces: Vec<String>
}

const INITIAL_POINTS: u32 = 10;

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

impl OSCashierHandler {
    pub fn register(&self, signerkey: String, payload: &OSCashierPayload, state: &mut OSCashierState) -> Result<(),ApplyError> {
        let username = payload.get_name();

        match state.does_entry_exist(&username) {
            Ok(exists) => {
                if exists {
                    return Err(ApplyError::InvalidTransaction("Already Exists".to_string()))
                }
            },
            Err(e) => {
                return Err(ApplyError::InternalError(format!(
                    "ContextError: {}", e.to_string()
                )));
            }
        };

        match state.get_state(username.clone(), signerkey) {
            Ok(internal_state) => {
                match state.set_state(&username, internal_state) {
                    Ok(_) => Ok(()),
                    Err(context_error) => Err(ApplyError::InternalError(format!(
                        "ContextError: {}", context_error.to_string()
                    )))
                }
            },
            Err(e) => Err(ApplyError::InternalError(format!("ContextError: {}", e.to_string())))
        }
    }

    pub fn plug_module(&self, signerkey: String, payload: &OSCashierPayload, state: &mut OSCashierState) -> Result<(),ApplyError> {
        let username = payload.get_name();

        match state.does_entry_exist(&username) {
            Ok(exists) => {
                if !exists {
                    return Err(ApplyError::InvalidTransaction("User doesn't exist".to_string()))
                }
            },
            Err(e) => {
                return Err(ApplyError::InternalError(format!(
                    "ContextError: {}", e.to_string()
                )));
            }
        };

        match state.get_state(username.clone(), signerkey) {
            Ok(mut internal_state) => {
                internal_state.add_mod(payload.get_module_name());
                // TODO: Update balance

                match state.set_state(&username, internal_state) {
                    Ok(_) => Ok(()),
                    Err(context_error) => Err(ApplyError::InternalError(format!(
                        "ContextError: {}", context_error.to_string()
                    )))
                }
            },
            Err(context_error) => Err(ApplyError::InternalError(format!(
                "ContextError: {}", context_error.to_string()
            )))
        }
    }

    pub fn unplug_module(&self, signerkey: String, payload: &OSCashierPayload, state: &mut OSCashierState) -> Result<(),ApplyError> {
        let username = payload.get_name();

        match state.does_entry_exist(&username) {
            Ok(exists) => {
                if !exists {
                    return Err(ApplyError::InvalidTransaction("User doesn't exist".to_string()))
                }
            },
            Err(e) => {
                return Err(ApplyError::InternalError(format!(
                    "ContextError: {}", e.to_string()
                )));
            }
        };

        match state.get_state(username.clone(), signerkey) {
            Ok(mut internal_state) => {
                internal_state.remove_mod(&payload.get_module_name());
                // TODO: Update balance

                match state.set_state(&username, internal_state) {
                    Ok(_) => Ok(()),
                    Err(context_error) => Err(ApplyError::InternalError(format!(
                        "ContextError: {}", context_error.to_string()
                    )))
                }
            },
            Err(context_error) => Err(ApplyError::InternalError(format!(
                "ContextError: {}", context_error.to_string()
            )))
        }
    }

    pub fn transfer(&self, signerkey: String, payload: &OSCashierPayload, state: &mut OSCashierState) -> Result<(),ApplyError> {
        let username = payload.get_name();
        let receiver = payload.get_receiver();

        // Check whether payer exists
        match state.does_entry_exist(&username) {
            Ok(exists) => {
                if !exists {
                    return Err(ApplyError::InvalidTransaction("Sending user doesn't exist".to_string()))
                }
            },
            Err(e) => {
                return Err(ApplyError::InternalError(format!(
                    "ContextError: {}", e.to_string()
                )));
            }
        };
        // Check whether receiver exists
        match state.does_entry_exist(&receiver) {
            Ok(exists) => {
                if !exists {
                    return Err(ApplyError::InvalidTransaction("Receiving user doesn't exist".to_string()))
                }
            },
            Err(e) => {
                return Err(ApplyError::InternalError(format!(
                    "ContextError: {}", e.to_string()
                )));
            }
        };

        let mut payer_state = match state.get_state(username.clone(), signerkey.clone()) {
            Ok(state) => state,
            Err(context_error) => return Err(ApplyError::InternalError(format!(
                "ContextError: {}", context_error.to_string()
            )))
        };
        let mut receiver_state = match state.get_state(receiver.clone(), signerkey.clone()) {     // TODO: It's public key won't be same and we don't care either
            Ok(state) => state,
            Err(context_error) => return Err(ApplyError::InternalError(format!(
                "ContextError: {}", context_error.to_string()
            )))
        };

        let payer_public_key = payer_state.get_key();

        if payer_public_key != signerkey {
            return Err(ApplyError::InvalidTransaction("You are not allowed to transfer someone else's points !".to_string()));
        };

        // The below are temporary, and in memory only, we will decrease balance, only when the payment was successful
        let transaction_amount = payload.get_amount();
        payer_state.dec_points(transaction_amount);
        receiver_state.add_points(transaction_amount);

        match state.set_state(&receiver, receiver_state) {
            Ok(_) => {
                match state.set_state(&username, payer_state) {
                    Ok(_) => Ok(()),
                    Err(context_error) => Err(ApplyError::InternalError(format!(
                        "ContextError: {}", context_error.to_string()
                    )))
                }
            },
            Err(context_error) => Err(ApplyError::InternalError(format!(
                "ContextError: {}", context_error.to_string()
            )))
        }
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
        let public_key = match header.as_ref() {
            Some(h) => h.signer_public_key.clone(),
            None => {
                return Err(ApplyError::InvalidTransaction(
                    "Invalid Header".to_string()
                ))
            }
        };

        let payload = match OSCashierPayload::from_bytes( &request.payload ) {
            Ok(payload) => payload,
            Err(apply_error) => {
                return Err(apply_error);
            }
        };

        let mut state = OSCashierState::new( context );

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
            println!("Received Payload:\n{:#?}", payload);
        }

        /*
         * Errors: ApplyError or ContextError
         * 
         * ApplyError: Either InvalidTransaction or InternalError... In invalid transaction, it will retry again once each second, or slightly faster, and will keep retry even if the tp unregisters then registers again.
         *                                                           In internal error, it retries EACH MILLISECONDS, don't return that, agar logs padhne layak chahiye to !
        */
        match payload.get_action() {
            Some(action) => match action {
                Actions::Register => self.register(public_key, &payload, &mut state),
                Actions::PlugMod => self.plug_module(public_key, &payload, &mut state),
                Actions::UnplugMod => self.unplug_module(public_key, &payload, &mut state),
                Actions::Transfer => self.transfer(public_key, &payload, &mut state)
            },
            None => {
                Err(ApplyError::InvalidTransaction("Unsupported Action specified".to_string()))
            }
        }
    }
}
