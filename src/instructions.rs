
//! Instruction parsing and printing

extern crate unescape;

use std::fmt;

/// Possible `vm` instructions
#[derive(Clone, Debug)]
pub enum Instruction {
    Pushi(i32),
    Pushn(i32),
    Pushg(usize),
    Pushs(String),
    Pusha(String),
    Pushgp,
    Call,
    Return,
    Start,
    Nop,
    Stop,
    Loadn,
    Writei,
    Writes,
    Read,
    Atoi,
    Padd,
    Add,
    Mul,
    Div,
    Mod,
    Storeg(usize),
    Storen,
    Equal,
    Inf,
    Infeq,
    Sup,
    Supeq,
    Jump(String),
    Jz(String),
    Err(String),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Pushi(ref val) => write!(f, "pushi {}", val),
            Instruction::Pushn(ref val) => write!(f, "pushn {}", val),
            Instruction::Pushg(ref val) => write!(f, "pushg {}", val),
            Instruction::Pushs(ref val) => write!(f, "pushs {}", val),
            Instruction::Pusha(ref val) => write!(f, "pusha {}", val),
            Instruction::Pushgp => write!(f, "pushgp"),
            Instruction::Call => write!(f, "call"),
            Instruction::Return => write!(f, "return"),
            Instruction::Start => write!(f, "start"),
            Instruction::Nop => write!(f, "nop"),
            Instruction::Stop => write!(f, "stop"),
            Instruction::Loadn => write!(f, "loadn"),
            Instruction::Writei => write!(f, "writei"),
            Instruction::Writes => write!(f, "writes"),
            Instruction::Read => write!(f, "read"),
            Instruction::Atoi => write!(f, "atoi"),
            Instruction::Padd => write!(f, "padd"),
            Instruction::Add => write!(f, "add"),
            Instruction::Mul => write!(f, "mul"),
            Instruction::Div => write!(f, "div"),
            Instruction::Mod => write!(f, "mod"),
            Instruction::Storeg(ref val) => write!(f, "storeg {}", val),
            Instruction::Storen => write!(f, "storen"),
            Instruction::Equal => write!(f, "equal"),
            Instruction::Inf => write!(f, "inf"),
            Instruction::Infeq => write!(f, "infeq"),
            Instruction::Sup => write!(f, "sup"),
            Instruction::Supeq => write!(f, "supeq"),
            Instruction::Jump(ref val) => write!(f, "jump {}", val),
            Instruction::Jz(ref val) => write!(f, "jz {}", val),
            Instruction::Err(ref val) => write!(f, "err {}", val),
        }
    }
}

impl Instruction {
    pub fn write_ln(&self) {
        match *self {
            Instruction::Writei | Instruction::Writes => println!(),
            _ => {}
        }
    }
}
