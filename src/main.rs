#[macro_use]
extern crate clap;

//use clap::{AppSettings, Clap};
use clap::{Arg, App};

/*
#[derive(Clap)]
#[clap(version = "0.271", author = "Aditya Gupta <ag15035@gmail.com>")]
#[clap(settings = AppSettings::ColoredHelp)]
*/

fn main() {
    /*
    let matches = clap_app!(oscashier => 
                    (version: crate_version!())
                    (about: "The OS Cashier (Rust)")
                    (@arg connect: -C --connect +takes_value)
                ).get_matches();
    */
    
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
             .multiple_occurences(true)
             .about("Sets verbosity level"))
        .subcommand(App::new("client")
                    .about("Client program")
                    // TODO: Dekho ye behtar hai ya jo bhi intkey use karta ho
                    )
        ).get_matches();

    
}

