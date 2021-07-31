use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OSCashierPayload {
    name: String,
    mods: Vec<String>,
    points: u32
}

impl OSCashierPayload {
    pub fn new( username: String, current_modules: Vec<String>, points: u32 ) -> OSCashierPayload {
        OSCashierPayload {
            name: username,
            mods: current_modules,
            points: points
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self).expect("[Client] Unable to Serialise Payload !")
    }
}
