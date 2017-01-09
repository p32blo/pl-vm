
extern crate vm;
extern crate clap;

use vm::Mode;
use clap::{App, Arg};

fn main() {
    let matches = App::new("vm")
        .about("A simple vm with debugger")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(Arg::from_usage("<input> 'Load the file in the vm'"))
        .arg(Arg::from_usage("-d --debug 'Start the vm in debug mode'"))
        .get_matches();

    let mode = if matches.is_present("debug") {
        Mode::Debug
    } else {
        Mode::Running
    };

    // There is a file argument
    if let Some(file) = matches.value_of("input") {
        // There are errors running the vm
        if let Err(ref e) = vm::start(&file, mode) {
            println!();
            println!("error: {}", e);
            for e in e.iter().skip(1) {
                println!("caused by: {}", e);
            }
        }
    }
}
