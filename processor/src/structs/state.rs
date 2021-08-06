use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use serde_derive::{Serialize, Deserialize};
use sawtooth_sdk::processor::handler::{TransactionContext};

mod util {
    pub fn get_timestamp_sec() -> u64 {
        chrono::Utc::now().timestamp() as u64
    }
}

#[derive(Debug,Serialize,Deserialize)]
struct InternalOSCashierState {
    name: String,
    points: u64,
    mods: BTreeMap<String,u64>, // {str, timepoint}, timepoint is "unix timestamp", and in seconds
}

impl InternalOSCashierState {
    pub fn from_bytes( state_bytes: &[u8] ) -> InternalOSCashierState {
        serde_cbor::from_str( std::str::from_utf8(state_bytes).to_string() ).unwrap()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self).expect("[InternalOSCashierState] Couldn't serialize state")
    }

    pub fn add_points(&mut self, points: u32) -> &InternalOSCashier {
        self.points = self.points + points;
        self
    }

    pub fn dec_points(&mut self, points: u32) -> &InternalOSCashier {
        // BUG: Skip checking bounds
        self.points = self.points - points;
        self
    }

    pub fn get_seconds_since_added(&self, module_name: &str) -> Result<u64,()> {
        let curr_timestamp = util::get_timestamp_sec();

        match self.mods.get(module_name) {
            Some(timestamp) => Ok(curr_timestamp - timestamp),
            None => Err(())
        }
    }

    pub fn add_mod(&mut self, module_name: &str) -> Result<(),()> {
        match self.mods.entry(module_name) {
            Entry::Occupied(_) => Err(()), // don't add if already present
            Entry::Vacant(e) => {
                e.into_mut().insert(util::get_timestamp_sec());
                Ok(())
            }
        }
    }

    pub fn remove_mod(&mut self, module_name: &str) -> Result<(),()> {
        match self.mods.remove(module_name) {
            Some(_) => Ok(()),  // key was present and removed
            None => Err(()) // key not present
        }
    }
}

pub struct OSCashierState {
    context: 'dyn TransactionContext,
    cache: BTreeMap<String,Vec<u8>> // this cache is valid, as only one tp is supposed to run for this os cashier version, no race problems, due to any other tp modifying it, rest internal race condition, we can always check later
}

impl OSCashierState {
    pub fn new( context: &dyn TransactionContext ) -> OSCashierState {
        OSCashierState {
            context,
            cache: BTreeMap::new()
        }
    }

    pub fn get_state(name: &str) -> Option<InternalState> {
        
        unimplemented!();
    }

    pub fn set_state(name: &str, updated_state: InternalState) -> Result<(),()> {
    
        unimplemented!();
    }

    pub fn does_entry_exist(name: &str) -> bool {
        let address = util::get_address(name);

        // just call context getstate then check if zero bytes or more
        context.get_state_entry(address).len() != 0
    }
}

