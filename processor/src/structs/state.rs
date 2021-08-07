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
    points: f64,
    mods: BTreeMap<String,u64>, // {str, timepoint}, timepoint is "unix timestamp", and in seconds
}

const DEFAULT_INIT_POINTS: f32 = 10.0;
const COIN_MULTIPLIER: f32 = 0.05;
impl _InternalOSCashierState {
    pub fn new(username: String, publickey: String) -> _InternalOSCashierState {
        _InternalOSCashierState {
            name: username,
            key: publickey,
            points: DEFAULT_INIT_POINTS as f64,
            mods: BTreeMap::new()
        }
    }

    pub fn from_bytes( state_bytes: &[u8] ) -> _InternalOSCashierState {
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

    pub fn get_points(&self) -> f64 {
        self.points
    }

    pub fn add_points(&mut self, points: f32) -> Result<(),()> {
        self.points += points as f64;
        Ok(())
    }

    pub fn dec_points(&mut self, points: f32) -> Result<(),()> {
        // BUG: Skip checking bounds
        self.points -= points as f64;
        Ok(())
    }

    pub fn get_seconds_since_added(&self, module_name: &str) -> Result<u64,()> {
        let curr_timestamp = util::get_timestamp_sec();

        match self.mods.get(module_name) {
            Some(timestamp) => Ok(curr_timestamp - timestamp),
            None => Err(())
        }
    }

    pub fn add_mod(&mut self, module_name: String) -> Result<(),()> {
        let performance_benefit = match self.get_module_rating(&module_name) { // From previous check, we know it exists in the array
            Some(rating) => rating,
            None => {
                println!("[{}/{}] No such module exists !", self.get_name(), module_name);
                return Err(());
            }
        };

        let transaction_cost = performance_benefit.abs();

        // We verified above that this is a valid module
        match self.mods.entry(module_name) {
            Entry::Occupied(_) => Err(()), // don't add if already present
            Entry::Vacant(e) => {
                e.insert(util::get_timestamp_sec());
                self.dec_points(transaction_cost)
            }
        }
    }

    pub fn remove_mod(&mut self, module_name: &str) -> Result<(),()> {
        let time_diff = match self.get_seconds_since_added(module_name) {
            Ok(diff) => diff,
            Err(_) => {
                return Err(())
            }
        };
        let performance_benefit = self.get_module_rating(module_name).expect("[FATAL]: No such module exists, It must not even have been added !"); // From previous check, we know it exists in the array

        let point_diff = COIN_MULTIPLIER * (time_diff as f32).sqrt() * performance_benefit;

        match self.mods.remove(module_name) {
            Some(_) => {  // key was present and removed
                if point_diff < 0.0 {
                    self.dec_points(point_diff.abs())
                } else {
                    self.add_points(point_diff)
                }
            }
            None => Err(()) // key not present
        }
    }
}

// FUTURE: For now, it's here, in future remove it
impl _InternalOSCashierState {
    // Making it a member function, since in future, it would be good if it changes dynamically based on the person him/herself
    pub fn get_module_rating(&self, module_name: &str) -> Option<f32> {
        match module_name {
            "slab_allocator" => Some(0.4),
            "slub_allocator" => Some(-0.1),
            "slob_allocator" => Some(-0.5),
            "buddy_allocator" => Some(0.2),
            _ => None
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

        prefix.to_string() + name_hash      // `String + &str` works fine !
    }

    pub fn get_state(&self, name: String, publickey: String) -> Result<_InternalOSCashierState, ContextError> {
        let address = OSCashierState::get_address(&name);

        // match self.cache.entry(name) {
        //     Entry::Vacant(_) => {},
        //     Entry::Occupied(e) => {
        //         return Ok( _InternalOSCashierState::from_bytes( e.get() ) )
        //     }
        // };
        match self.context.get_state_entry(&address) {
            Ok(o) => match o {
                Some(state_bytes) => {
                    // TODO: Find some way to update the cache after get operations too
                    // self.cache.insert(name.to_string(), _updated_state);
                    Ok(_InternalOSCashierState::from_bytes(&state_bytes))
                },
                None => Ok(_InternalOSCashierState::new(name, publickey))
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn set_state(&mut self, name: &str, updated_state: _InternalOSCashierState) -> Result<(),ContextError> {
        self.context.set_state_entry(OSCashierState::get_address(name), updated_state.to_bytes())
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

