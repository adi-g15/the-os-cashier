extern crate clap;
#[macro_use]
extern crate log;
extern crate log4rs;

//use clap::{AppSettings, Clap};
use clap::{Arg, App};
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

use std::process;   // for process::exit()

use sawtooth_sdk::processor::TransactionProcessor;

mod handler;
use handler::OSCashierHandler;

fn main() {
    let matches = App::new("OS Cashier Transaction Processor")
        .version("0.271")
        .author("Aditya Gupta <ag15035@gmail.com>")
        .about("Based on a operating system, and the user/entity that improves performace is rewarded, like a ML model, to improve choices over time (if user/entity observes this profit loss pattern)")
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

     
     let endpoint = matches
          .value_of("connect")
          .unwrap_or("tcp://validator:4004");

     let console_log_level = match matches.occurrences_of("v") {
          0 => LevelFilter::Warn,
          1 => LevelFilter::Info,
          2 => LevelFilter::Debug,
          _ => LevelFilter::Trace
     };

     // for logging, taken from intkey code
     let stdout = ConsoleAppender::builder()
          .encoder(Box::new(PatternEncoder::new(
               "{h({l:5.5})} | {({M}:{L}):20.20} | {m}{n}",
          )))
          .build();

     let config = match Config::builder()
          .appender(Appender::builder().build("stdout", Box::new(stdout)))
          .build(Root::builder().appender("stdout").build(console_log_level))
          {
               Ok(x) => x,
               Err(e) => {
                    for err in e.errors().iter() {
                         info!("Configuration error: {}", err.to_string());
                    }
                    process::exit(1);
               }
          };

     match log4rs::init_config(config) {
          Ok(_) => (),
          Err(e) => {
               info!("Configuration error: {}", e.to_string());
               process::exit(1);
          }
     };

     let handler = OSCashierHandler::new();
     let mut processor = TransactionProcessor::new(endpoint);
     processor.add_handler( &handler );
     processor.start();
}

