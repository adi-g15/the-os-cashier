use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OSCashierPayload {
    pub name: String,
    pub curr_mods: Vec<String>,
    pub points: u32,
    // balance: u32
    // timepoint    // this field will be added by the tp
}

impl OSCashierPayload {
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self).expect("[Client] Unable to Serialise Payload !")
    }
}
