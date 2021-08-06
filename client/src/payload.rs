use serde_derive::{Serialize, Deserialize};
use std::collections::btree_map::BTreeMap;

pub enum Actions {
    Register,
    PlugMod,    // add
    UnplugMod,  // remove
    List,
    ListMod
}

impl Actions {
    pub fn to_string(action: Actions) -> String {
        match action {
            Actions::Register => "Register",
            Actions::PlugMod => "PlugMod",
            Actions::UnplugMod => "UnplugMod",
            Actions::ListMod => "ListMod",
        }.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OSCashierPayload {
    action: Actions,
    name: String,
    receiver: String,
    amount: u32,
    module: String
}

impl OSCashierPayload {
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self).expect("[Client] Unable to Serialise Payload !")
    }

    pub fn new( payload_bytes: &[u8] ) -> Result<OSCashierPayload,ApplyError> {

        let payload = match serde_cbor::from_str( std::str::from_utf8(payload_bytes).to_string() ).unwrap();

       unimplemented!();
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_module_name(&self) -> String {
        self.module.clone();
    }

    pub fn get_seconds_since_added(&self, module_name: &str) -> Result<u64,()> {
        let current_time = chrono::Utc::now().timestamp();  // taking the timestamp before going for any long iterations

        match self.curr_mods.get(module_name) {
            Some(timestamp) => Ok(timestamp - current_time as u64),
            None => {
                // module not found
                Err(())
            }
        }
    }
}
