#[macro_use]
extern crate clap;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.271", author = "Aditya Gupta <ag15035@gmail.com>")]
#[clap(settings = AppSettings::ColoredHelp)]
struct CLIOpts {
    #[clap(short, long, default value = )]
    // TODO: adi Hona kya chahiye isme ?
}

fn main() {
    
}

