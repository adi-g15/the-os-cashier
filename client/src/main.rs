#![allow(unused_variables)]
use clap::{AppSettings, clap_app};
use std::process;

mod util;
mod client;
mod payload;
use client::OSCashierClient;

enum VerbosityLevel {
    Normal,
    Debug,
    Highest
}

fn main() {
    let matches = clap_app!(The_OS_Cashier => 
                    (setting: AppSettings::ColoredHelp)
                    (version: "0.1")
//                    (author: "Aditya Gupta <ag15035@gmail.com>")
                    (about: "The Blockchain is the Distributed Computer...\nValidator is the CPU...\nYou are the kernel")
                    (@arg url: --url +takes_value "URL of the REST API")
                    (@subcommand list => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Lists current users (with any plugged modules) or modules")
                        (@arg modules: "(Optional Arg) List modules")
                     )
                    (@subcommand reg => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Register a new user")
                        (@arg user: +required "Username of the new user")
                     )
                    (@subcommand plug => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Plug a module")
                        (@arg user: "Username of user") // Not required, intentionally
                        (@arg module: +required "Name of pre-available module")
                     )
                    (@subcommand unplug => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Unplug a module")
                        (@arg user: "Username of user")
                        (@arg module: +required "Name of pre-available module")
                     )
                    ).get_matches();

    let rest_api_url = matches.value_of("url").unwrap_or("http://localhost:8008");

    let client = OSCashierClient::new(
        rest_api_url.to_string()
    );

    match matches.subcommand() {
        Some(cmd) => {
            match cmd.0 {
                "list" => {
                    client.list(cmd.1.is_present("modules"));
                },
                "reg" => {
                    match cmd.1.value_of("user") {
                        Some(username) => client.reg(username),
                        None => {
                            println!("Username required !");
                            process::exit(1);
                        }
                    }
                },
                "plug" => {
                    let username = match cmd.1.value_of("user") {
                        Some(username) => username.to_string(),
                        None => whoami::username()
                    };

                    match cmd.1.value_of("module") {
                        Some(module_name) => client.plug(&username, module_name),
                        None => {
                            println!("Module name required !\nTip: Use \"list modules\" subcommand");
                            process::exit(1);
                        }
                    }
                },
                "unplug" => {
                    let username = match cmd.1.value_of("user") {
                        Some(username) => username.to_string(),
                        None => whoami::username()
                    };

                    match cmd.1.value_of("module") {
                        Some(module_name) => client.unplug(&username, module_name),
                        None => {
                            println!("Module name required !\nTip: Use \"list modules\" subcommand");
                            process::exit(1);
                        }
                    }
                },
                _ => {
                    println!("Unrecognised Operation !");
                    process::exit(1);
                }
            }
        },
        None => {
            println!("No Operation specified ! Use --help to see available options");
            process::exit(0);
        }
    }

    /* FUTURE: Instead of try catch block, in the function that interacts with the API, call
     * process::exit there
     * Later, if needed create a try catch block here
     */

    /* [DONE]: Step 1- Create a Client object, and call respective function for the operation, for
     * eg. reg, list etc.
     */


    /* TODO: Step 2- Inside those function, make a call to "_send_transaction" that creates the
     * Transaction, Header, Payload objects (and serialise this payload object), and finally make
     * the REST API call, for now lets say that's "send_rest_api_call"
     */


    /* TODO: Step 3- Inside _send_transaction itself, batch all transactions, say with a
     * "create_batch_list([transaction])" function, then this batch list will be serialised to a
     * string, and sent to send_rest_api_call
     */


    /* TODO: Step 4- Inside the "create_batch_list", a batch header is created, and signed, and
     * then the received transactions array is put into a newly created 'Batch' object
     */


    /* TODO: Step 5- Inside the _send_request, use the url passed with --url, NOTE: For now, take
     * it as "http://rest-api:8008"
     */

}
