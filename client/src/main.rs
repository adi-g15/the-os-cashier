use clap::{AppSettings, clap_app};
use std::process;

mod util;
mod client;
mod payload;
use client::OSCashierClient;

fn main() {
    let matches = clap_app!(The_OS_Cashier => 
                    (setting: AppSettings::ColoredHelp)
                    (version: "0.1")
//                    (author: "Aditya Gupta <ag15035@gmail.com>")
                    (about: "The Blockchain is the Distributed Computer...\nValidator is the CPU...\nYou are the kernel")
                    (@arg url: --url +takes_value "URL of the REST API")
                    (@subcommand list => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Lists available modules")
                        // (about: "Lists current users (with any plugged modules) or modules")
                        // (@arg modules: "(Optional Arg) List modules")
                     )
                    (@subcommand reg => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Register a new user")
                        (@arg user: +required "Username of the new user")
                     )
                    (@subcommand plug => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Plug a module")
                        (@arg user: +required "Username of user") // Not required, intentionally, TODO: For now making it required, due to clap requires optionals to be at last
                        (@arg module: +required "Name of pre-available module")
                     )
                    (@subcommand unplug => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Unplug a module")
                        (@arg user: +required "Username of user")
                        (@arg module: +required "Name of pre-available module")
                     )
                    (@subcommand transfer => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Transfer asset")
                        (@arg sender: +required "Username that sends the coins")
                        (@arg reciever: +required "Username that receives the coins")
                        (@arg amount: +required "Transaction amount")
                     )
                    ).get_matches();

    let rest_api_url = matches.value_of("url").unwrap_or("http://localhost:8008");

    let client = OSCashierClient::new(
        rest_api_url.to_string()
    );

    /* TODO:
     * Currently there is no good use of the key and signing, as anyone can plug/unplug or transfer in other's name... find some ways
     */

    match matches.subcommand() {
        Some(cmd) => {
            match cmd.0 {
                "list" => {
                    // client.list(cmd.1.is_present("modules"));
                    client.list_modules();
                },
                "reg" => {
                    match cmd.1.value_of("user") {
                        Some(username) => client.reg(username.to_string()),
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
                        Some(module_name) => client.plug(username, module_name.to_string()),
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
                        Some(module_name) => client.unplug(username, module_name.to_string()),
                        None => {
                            println!("Module name required !\nTip: Use \"list modules\" subcommand");
                            process::exit(1);
                        }
                    }
                },
                "transfer" => {
                    let sender = match cmd.1.value_of("sender") {
                        Some(username) => username.to_string(),
                        None => whoami::username()
                    };

                    match cmd.1.value_of("receiver") {
                        Some(receiver) => {
                            match cmd.1.value_of("amount") {
                                Some(amount) => client.transfer(sender, receiver.to_string(), amount.parse().unwrap()),    // convert amount from &str to number
                                None => {
                                    println!("Wrong request: Pass transaction amount!");
                                    process::exit(1);
                                }
                            }
                        },
                        None => {
                            println!("Wrong request: Pass receiver username!");
                            process::exit(1);
                        }
                    }

                }
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
     * CURRENT: Currently simply having 'panic', if ANYTHING fails the client will end... sun ke achha to nahi lag raha bhai ye :'D
     */

    // NOTE: Now all these steps have been implemented, 

    /* [DONE]: Step 1- Create a Client object, and call respective function for the operation, for
     * eg. reg, list etc.
     */


    /* [DONE] TODO: Step 2- Inside those function, make a call to "_send_transaction" that creates the
     * Transaction, Header, Payload objects (and serialise this payload object), and finally make
     * the REST API call, for now lets say that's "send_rest_api_call"
     */


    /* [DONE] TODO: Step 3- Inside _send_transaction itself, batch all transactions, say with a
     * "create_batch_list([transaction])" function, then this batch list will be serialised to a
     * string, and sent to send_rest_api_call
     */


    /* [DONE] TODO: Step 4- Inside the "create_batch_list", a batch header is created, and signed, and
     * then the received transactions array is put into a newly created 'Batch' object
     */


    /* TODO: Step 5- Inside the _send_request, use the url passed with --url, NOTE: For now, take
     * it as "http://rest-api:8008"
     */

}
