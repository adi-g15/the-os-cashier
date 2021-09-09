use clap::{Arg,App};
use sawtooth_sdk::consensus::zmq_driver;

mod engine;
use engine::CustomEngine;

fn main() {
    let matches = App::new("Custom consensus engine")
        .version("0.271")
        .author("Aditya Gupta <ag15035@gmail.com>")
        .about("Just trying")
        .arg(Arg::new("connect")
             .short('C')
             .long("connect")
             .value_name("connect")
             .about("Validator endpoint")
             .takes_value(true))
        .arg(Arg::new("v")
             .short('v')
             .multiple_occurrences(true)
             .about("Sets verbosity level"))
        .get_matches();

    let endpoint = matches.value_of("connect").unwrap_or("tcp://localhost:5050");

    // Create a new ZMQ-based consensus engine driver (and a handle for stopping it)
    let (driver, _stop) = zmq_driver::ZmqDriver::new();

    match driver.start(endpoint, CustomEngine::new()) {
        Ok(()) => (),
        Err(e) => {
            _stop.stop();

            panic!("{}",e);
        }
    }
}
