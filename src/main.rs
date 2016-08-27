
extern crate vm;
extern crate ansi_term;
use ansi_term::Colour::Blue;

use std::error::Error;

fn main() {

    let args = std::env::args().skip(1);

    for file in args {
        let file_col = Blue.paint(file.to_string());
        if let Err(e) = vm::start(&file) {
            println!("{}: {}", file_col, e.description());
        }
    }
}
