use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use serde_derive::{Serialize, Deserialize};
use sawtooth_sdk::processor::handler::{TransactionContext, ContextError};

mod util {
    pub fn get_timestamp_sec() -> u64 {
        chrono::Utc::now().timestamp() as u64
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct _InternalOSCashierState {
    name: String,
    key: String,    // public key
    points: u64,
    mods: BTreeMap<String,u64>, // {str, timepoint}, timepoint is "unix timestamp", and in seconds
}

const DEFAULT_INIT_POINTS: u32 = 10;
impl _InternalOSCashierState {
    pub fn new(username: String, publickey: String) -> _InternalOSCashierState {
        _InternalOSCashierState {
            name: username,
            key: publickey,
            points: DEFAULT_INIT_POINTS as u64,
            mods: BTreeMap::new()
        }
    }

    pub fn from_bytes( state_bytes: &[u8] ) -> _InternalOSCashierState {
        println!("Got state bytes -> {:#?}", state_bytes);
        serde_cbor::from_slice( state_bytes ).unwrap()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self).expect("[_InternalOSCashierState] Couldn't serialize state")
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_points(&self) -> u64 {
        self.points
    }

    pub fn add_points(&mut self, points: u32) -> &_InternalOSCashierState {
        self.points += points as u64;
        self
    }

    pub fn dec_points(&mut self, points: u32) -> &_InternalOSCashierState {
        // BUG: Skip checking bounds
        self.points -= points as u64;
        self
    }

    pub fn get_seconds_since_added(&self, module_name: &str) -> Result<u64,()> {
        let curr_timestamp = util::get_timestamp_sec();

        match self.mods.get(module_name) {
            Some(timestamp) => Ok(curr_timestamp - timestamp),
            None => Err(())
        }
    }

    pub fn add_mod(&mut self, module_name: String) -> Result<(),()> {
        match self.mods.entry(module_name) {
            Entry::Occupied(_) => Err(()), // don't add if already present
            Entry::Vacant(e) => {
                e.insert(util::get_timestamp_sec());
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

pub struct OSCashierState<'a> {
    context: &'a mut dyn TransactionContext,
    cache: BTreeMap<String,Vec<u8>> // this cache is valid, as only one tp is supposed to run for this os cashier version, no race problems, due to any other tp modifying it, rest internal race condition, we can always check later
}

const FAMILY_NAME: &str = "os-cashier";
impl<'a> OSCashierState<'a> {
    pub fn new( context: &'a mut dyn TransactionContext ) -> OSCashierState {
        OSCashierState {
            context,
            cache: BTreeMap::new()
        }
    }

    pub fn get_address(name: &str) -> String {
        let prefix = &hex::encode( openssl::sha::sha512(FAMILY_NAME.as_bytes() ))[0..6];
        let name_hash = &hex::encode( openssl::sha::sha512(name.as_bytes()) )[64..];

        println!("Prefix is: {}", prefix);
        println!("Hash for name: {} is {}, length: {}", name, name_hash, name_hash.len());

        prefix.to_string() + name_hash      // `String + &str` works fine !
    }

    pub fn get_state(&self, name: String, publickey: String) -> Result<_InternalOSCashierState, ContextError> {
        let address = OSCashierState::get_address(&name);

        match self.context.get_state_entry(&address) {
            Ok(o) => match o {
                Some(state_bytes) => {
                    Ok(_InternalOSCashierState::from_bytes(&state_bytes))
                },
                None => Ok(_InternalOSCashierState::new(name, publickey))
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn set_state(&mut self, name: &str, _updated_state: _InternalOSCashierState) -> Result<(),ContextError> {
        // AutorizationError: Tried to set unauthorized addresses: [address: "user" data: "\244dnameduserckeyxB02cf2a47c3f72b9bfc944007d32f248c44b25234edc2bc76e6c79d3ad9bd8f5716fpoints\ndmods\240"]
        self.context.set_state_entry(OSCashierState::get_address(name), _updated_state.to_bytes())
    }

    pub fn does_entry_exist(&self, name: &str) -> Result<bool, ContextError> {
        // just call context getstate then check if zero bytes or more
        match self.context.get_state_entry(
            &OSCashierState::get_address(name)
        ) {
            Ok(entry) => match entry {
                Some(_) => Ok(true),
                None => Ok(false)
            },
            Err(e) => Err(e)
        }
    }
}

