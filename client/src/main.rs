use clap::{AppSettings, clap_app};

fn main() {
    let matches = clap_app!(The_OS_Cashier => 
                    (setting: AppSettings::ColoredHelp)
                    (version: "0.1")
//                    (author: "Aditya Gupta <ag15035@gmail.com>")
                    (about: "The Blockchain is the Distributed Computer...\nValidator is the CPU...\nYou are the kernel")
                    (@arg verbose: -v --verbose "Be more verbose")
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
                        (@arg user: +required "Username of user")
                        (@arg module: +required "Name of pre-available module")
                     )
                    (@subcommand unplug => 
                        (setting: AppSettings::ColoredHelp)
                        (about: "Unplug a module")
                        (@arg user: +required "Username of user")
                        (@arg module: +required "Name of pre-available module")
                     )
                    ).get_matches();

    /* TODO: Instead of try catch block, in the function that interacts with the API, call
     * process::exit there
     * Later, if needed create a try catch block here
     */

    
    /* TODO: Step 1- Create a Client object, and call respective function for the operation, for
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
