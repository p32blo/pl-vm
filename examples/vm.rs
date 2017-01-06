
extern crate vm;

fn main() {
    // There is a file argument
    if let Some(file) = std::env::args().skip(1).next() {
        // There are errors running the vm
        if let Err(ref e) = vm::start(&file) {
            println!("error: {}", e);
            for e in e.iter().skip(1) {
                println!("caused by: {}", e);
            }
        }
    }
}
