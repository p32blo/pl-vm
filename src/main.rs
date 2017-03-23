
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

#![recursion_limit="128"]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate error_chain;
extern crate clap;

mod vm;
mod instructions;
mod commands;
mod parse;

use vm::Mode;
use clap::{App, Arg};

use instructions::Instruction;

named!(digit1<&str, &str>, flat_map!(take!(1), nom::digit));
named!(alpha1<&str, &str>, flat_map!(take!(1), nom::alpha));
named!(ident1<&str,&str>, alt!(tag!("_") | alpha1));
named!(ident2<&str,Vec<&str>>, many0!(alt!(digit1 | alpha1 | tag!("_") | tag!("'"))));
named!(ident<&str, String>, do_parse!(id1: ident1 >> id2: ident2 >> (id1.to_string() + &id2.join(""))));

named!(code<&str, Vec<Instruction>>, many0!(ws!(instr)));

// named!(instr<&str, Instruction>, 
//     alt!(
//         instr_atom
//         | do_parse!(
//             id: ident >>
//             tag!(":") >>
//             (Instruction::Label(id))
//         ) 
//         | do_parse!(
//             id: instr_int >>
//             int: nom::digit >>
//             (id)
//         )
//     )
// );


named!(instr_atom<&str, Instruction>, 
    alt!(
        tag!("add") => { |_| Instruction::Add } 
        | tag!("sub") => { |_| Instruction::Sub } 
        | tag!("mul") => { |_| Instruction::Mul } 
        | tag!("div") => { |_| Instruction::Div } 
        | tag!("mod") => { |_| Instruction::Mod } 
        | tag!("not") => { |_| Instruction::Not } 
        | tag!("inf") => { |_| Instruction::Inf }  
        | tag!("infeq") => { |_| Instruction::Infeq } 
        | tag!("sup") => { |_| Instruction::Sup } 
        
        | tag!("supeq") => { |_| Instruction::Supeq }  
        //| tag!("fadd") => { |_| Instruction::FAdd }  
        //| tag!("fsub") => { |_| Instruction::FSub } 
        //| tag!("fmul") => { |_| Instruction::FMull }  | tag!("fdiv") | tag!("fcos") | tag!("fsin") |
        // tag!("finf") | tag!("finfeq") | tag!("fsup") | tag!("fsupeq") | tag!("concat") | tag!("equal") | tag!("atoi") | tag!("atof") |
        //tag!("itof") | tag!("ftoi") | tag!("stri") | tag!("strf") |
        //tag!("pushsp") | tag!("pushfp") | tag!("pushgp") | tag!("loadn") | tag!("storen") | tag!("swap") |
        //tag!("writei") | tag!("writef") | tag!("writes") | tag!("read") | tag!("call") | tag!("return") |
        //tag!("drawpoint") | tag!("drawline") | tag!("drawcircle") |
        //tag!("cleardrawingarea") | tag!("opendrawingarea") | tag!("setcolor") | tag!("refresh") |
        //tag!("start") | tag!("nop") | tag!("stop") | tag!("allocn") | tag!("free") | tag!("dupn") | tag!("popn") |
        //tag!("pushi") | tag!("pushn") | tag!("pushg") | tag!("pushl") | tag!("load") |
        //tag!("dup") | tag!("pop") | tag!("storel") | tag!("storeg") | tag!("alloc")
    )
);

named!(instr_int<&str, Instruction>,
    alt!(
        tag!("pushi") => { |_| Instruction::Pushi } 
        | tag!("pushn") => { |_| Instruction::Pushn(0) }
        | tag!("pushg") => { |_| Instruction::Pushg(0) } 
//        | tag!("pushl") => { |_| Instruction::pushl } 
//        | tag!("load") => { |_| Instruction::Load } 
        
//        | tag!("dup") => { |_| Instruction::Dup } 
//        | tag!("pop") => { |_| Instruction::Pop } 
//        | tag!("storel") => { |_| Instruction::Storel } 
        | tag!("storeg") => { |_| Instruction::Storeg } 
//        | tag!("alloc") => { |_| Instruction::Alloc } 
    )
);

mod errors {
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

    println!("{:?}", code("test: add"));

    // There is a file argument
    if let Some(file) = matches.value_of("input") {
        // There are errors running the vm
        if let Err(ref e) = vm::start(&file, mode) {
            errors::print_errors(e);
        }
    }
}
