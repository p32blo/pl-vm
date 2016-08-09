
extern crate vm;

fn main() {

    let mut args = std::env::args().skip(1);

    match &args.len() {
        &1 => vm::start(args.next().unwrap()),
        _ => println!("Provide a file for execution")
    }
}
