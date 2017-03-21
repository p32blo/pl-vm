
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
extern crate ansi_term;
extern crate clap;

mod vm;
mod instructions;
mod commands;

use vm::Mode;
use clap::{App, Arg};

/// Error handling
mod errors {
    use ansi_term::Colour::Red;

    error_chain!{
        errors {
            IllegalOperand {
                description("Triggered when the value(s) on the stack are not of the expected nature")
                display("Illegal Operand")
            }
            SegmentationFault {
                description("Triggered for access to an illegal area of ​​the code, stack, or one of two heaps")
                display("Segmentation Fault")
            }
            StackOverflow {
                description("Triggered for any attempt to add to the top of a full stack (execution stack or call stack)")
                display("Stack Overflow")
            }
            DivisionByZero {
                description("Triggered in case of division (integer) by zero")
                display("Division By Zero")
            }
            Error(message: String) {
                description("Triggered when the err statement is executed")
                display("{}", Red.paint(message.clone()))
            }
            Anomaly {
                description("This error must never occur. If so please report it!")
                display("Anomaly")
            }
        }
    }

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
        println!("\n{}", Red.paint(e.to_string() + ":"));
        for e in e.iter().skip(1) {
            println!("{}{}", Red.paint("caused by: "), e);
        }
    }
}

fn main() {
    let matches = App::new("pl-vm")
        .about("A simple vm with debug support")
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
