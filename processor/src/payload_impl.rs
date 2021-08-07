pub use crate::structs::payload::{OSCashierPayload,Actions};
use sawtooth_sdk::processor::handler::ApplyError;

impl OSCashierPayload {
    pub fn from_bytes( payload_bytes: &[u8] ) -> Result<OSCashierPayload,ApplyError> {
        match serde_cbor::from_slice( payload_bytes ) {
            Ok(obj) => Ok(obj),
            Err(e) => Err(ApplyError::InvalidTransaction(format!("Error: Failed parsing payload - {:#?}",e)))
        }
    }
}
