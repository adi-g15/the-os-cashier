use sawtooth_sdk::consensus::engine::{Engine,StartupState,Update,Error};
use sawtooth_sdk::consensus::service::Service;
use std::sync::mpsc::Receiver;

pub struct CustomEngine {

}

impl CustomEngine {
    pub fn new() -> CustomEngine {
        unimplemented!();
    }
}

/// We need to implement consensus::engine::Engine for each consensus engine
/// To implement: start`, `version`, `name`, `additional_protocols`
impl Engine for CustomEngine {
    fn start(
        &mut self,
        updates: Receiver<Update>,
        service: Box<dyn Service>,
        startup_state: StartupState) -> Result<(), Error> {

            Err(Error::SendError("Unimplemented".to_string()))
    }

    fn version(&self) -> String {
        "0.1".to_string()
    }

    fn name(&self) -> String {
        "CustomEngine".to_string()
    }

    fn additional_protocols(&self) -> Vec<(String,String)> {
        vec![]
    }
}
