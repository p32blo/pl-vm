
extern crate vm;
extern crate ansi_term;

use std::process;
use ansi_term::Colour::{Red, Blue};

fn main() {
    let args = std::env::args().skip(1);

    for file in args {

        println!();
        println!("{} {}", Blue.paint("Begin Execution:"), file);

        if let Err(ref e) = vm::start(&file) {
            println!("{} {}", Red.paint("error:"), e);

            for e in e.iter().skip(1) {
                println!("{} {}", Red.paint("caused by:"), e);
            }

            // The backtrace is not always generated. Try to run this example
            // with `RUST_BACKTRACE=1`.
            if let Some(backtrace) = e.backtrace() {
                println!("backtrace: {:?}", backtrace);
            }
        }

        println!();
        println!("{} {}", Blue.paint("End Execution:"), file);
    }
    process::exit(0);
}
