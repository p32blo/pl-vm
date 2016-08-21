
extern crate vm;

use std::error::Error;

fn main() {

    let mut args = std::env::args().skip(1);

    match args.by_ref().len() {
        1 => {
            let file = args.next().unwrap();
            if let Err(e) = vm::start(&file) {
                println!("error: {}", e.description());
            }
        }
        _ => println!("error: no file provided"),
    }
}
