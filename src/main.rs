
//! A simple `vm` with debug support
//!
//! Executing the `vm`:
//!
//! ```
//! pl-vm <file>
//! ```
//! Executing the `vm` in debug mode:
//!
//! ```
//! pl-vm --debug <file>
//! ```
//!


#[macro_use]
extern crate error_chain;

extern crate clap;
use clap::{App, Arg};

mod vm;
mod instructions;
mod commands;

use vm::Mode;

pub mod errors {
    //! Error handling
    error_chain!{}

    /// Print the error chain in oneline
    pub fn print_errs(e: &Error) {
        print!("\t{}. ", e);
        for e in e.iter().skip(1) {
            print!("{}. ", e);
        }
        println!();
    }

    /// Print a multiline error chain
    pub fn print_errors(e: &Error) {
        println!("error: {}", e);
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }
    }
}

fn main() {
    let matches = App::new("pl-vm")
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
            errors::print_errors(e);
        }
    }
}
