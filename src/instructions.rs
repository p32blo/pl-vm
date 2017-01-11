
extern crate unescape;

use std::fmt;
use std::str::FromStr;

use errors::*;

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


impl FromStr for Instruction {
    type Err = Error;
    fn from_str(instr: &str) -> Result<Instruction> {
        let (inst, val) = Self::decode(instr);

        let val_s = |val: Option<String>| -> Result<String> {
            val.ok_or(format!("No value found for '{}' instruction", inst).into())
        };
        let val_i = |val_s: Result<String>| -> Result<i32> {
            val_s.and_then(|x| x.parse().chain_err(|| "value is not a integer"))
        };
        let val_u = |val_s: Result<String>| -> Result<usize> {
            val_s.and_then(|x| x.parse::<usize>().chain_err(|| "value is not a positive integer"))
        };

        let res = match inst.as_ref() {
            "pushi" => Instruction::Pushi(val_i(val_s(val))?),
            "pushn" => Instruction::Pushn(val_i(val_s(val))?),
            "pushg" => Instruction::Pushg(val_u(val_s(val))?),
            "pushs" => Instruction::Pushs(Self::remove_quotes(&val_s(val)?)),
            "pusha" => Instruction::Pusha(val_s(val)?),
            "pushgp" => Instruction::Pushgp,
            "call" => Instruction::Call,
            "return" => Instruction::Return,
            "start" => Instruction::Start,
            "nop" => Instruction::Nop,
            "stop" => Instruction::Stop,
            "loadn" => Instruction::Loadn,
            "writei" => Instruction::Writei,
            "writes" => Instruction::Writes,
            "read" => Instruction::Read,
            "atoi" => Instruction::Atoi, 
            "padd" => Instruction::Padd,
            "add" => Instruction::Add,
            "mul" => Instruction::Mul,
            "div" => Instruction::Div,
            "mod" => Instruction::Mod,
            "storeg" => Instruction::Storeg(val_u(val_s(val))?),
            "storen" => Instruction::Storen,
            "equal" => Instruction::Equal,
            "inf" => Instruction::Inf,
            "infeq" => Instruction::Infeq,
            "sup" => Instruction::Sup,
            "supeq" => Instruction::Supeq,
            "jump" => Instruction::Jump(val_s(val)?),
            "jz" => Instruction::Jz(val_s(val)?),
            "err" => Instruction::Err(Self::remove_quotes(&val_s(val)?)),
            _ => panic!(format!("Instruction not found: {}", inst)),
        };
        Ok(res)
    }
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
    fn decode(inst_ref: &str) -> (String, Option<String>) {
        let find = inst_ref.find(' ');
        match find {
            None => (inst_ref.to_string(), None),
            Some(f) => {
                let (inst, val) = inst_ref.split_at(f);
                (inst.to_string(), Some(val.trim().to_string()))
            }
        }
    }

    fn remove_quotes(string: &str) -> String {
        let mut val = string.to_string();

        // Assumes well formed strings with both quotes
        val.remove(0);
        val.pop().unwrap();

        unescape::unescape(&val).unwrap()
    }

    pub fn write_ln(&self) {
        match *self {
            Instruction::Writei | Instruction::Writes => println!(),
            _ => {}
        }
    }
}
