use serde_derive::{Serialize, Deserialize};

use std::fmt;

pub enum Actions {
    Register,
    PlugMod,    // add
    UnplugMod,  // remove
    // List,
    ListMod,
    Transfer
}

impl Actions {
    pub fn from_string(action: &str) -> Option<Actions> {
        match action {
            "Register" => Some(Actions::Register),
            "PlugMod" => Some(Actions::PlugMod),
            "UnplugMod" => Some(Actions::UnplugMod),
            "ListMod" => Some(Actions::ListMod),
            "Transfer" => Some(Actions::Transfer),
            _ => None
        }
    }
}

impl ToString for Actions {
    fn to_string(&self) -> String {
        match self {
            Actions::Register => "Register",
            Actions::PlugMod => "PlugMod",
            Actions::UnplugMod => "UnplugMod",
            Actions::ListMod => "ListMod",
            Actions::Transfer => "Transfer"
        }.to_string()
    }
}

impl fmt::Debug for Actions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt( format_args!( "Actions::{}", &self.to_string() ) )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OSCashierPayload {
    action: String,
    name: String,
    receiver: String,
    amount: u32,
    module: String
}

impl OSCashierPayload {
    pub fn new(action: Actions, username: String) -> OSCashierPayload {
        OSCashierPayload {
            action: action.to_string(),
            name: username,
            receiver: String::from(""),
            amount: 0,
            module: String::from("")
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self).expect("[Client] Unable to Serialise Payload !")
    }

    pub fn set_receiver(&mut self, receiver: String) {
        self.receiver = receiver;
    }

    pub fn set_amount(&mut self, amount: u32) {
        self.amount = amount;
    }

    pub fn set_module(&mut self, module_name: String) {
        self.module = module_name;
    }

    #[allow(unused)]
    pub fn get_action(&self) -> Option<Actions> {
        Actions::from_string(&self.action)
    }

    #[allow(unused)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[allow(unused)]
    pub fn get_module_name(&self) -> String {
        self.module.clone()
    }
}
