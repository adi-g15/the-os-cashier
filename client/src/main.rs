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
}
