use serde_derive::{Deserialize};
use std::collections::btree_map::BTreeMap;

pub enum Actions {
    Register,
    PlugMod,    // add
    UnplugMod,  // remove
    List,
    ListMod
}

#[derive(Debug, Deserialize)]
pub struct OSCashierPayload {
    action: String,
    name: String,
    curr_mods: BTreeMap<String,u64>,    // the value is timestamp, of the moment the module was added (timestamp - number of non-leap seconds since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp"))
    points: u32,
    // balance: u32
    timepoints: Vec<u64>    
}

impl OSCashierPayload {
    pub fn new( _payload_bytes: &[u8] ) -> OSCashierPayload {

        // verify whether timepoints, and curr_mods vector are same in length
        unimplemented!();
    }

    pub fn get_action(&self) -> Result<Actions,()> {
        match self.action.as_str() {
            "Register" => Ok(Actions::Register),
            "PlugMod" => Ok(Actions::PlugMod),
            "UnplugMod" => Ok(Actions::UnplugMod),
            "List" => Ok(Actions::List),
            "ListMod" => Ok(Actions::ListMod),
            _ => {
                Err(())
            }
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_current_modules(&self) -> Vec<&str> {
        self.curr_mods.keys();

        unimplemented!();
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
