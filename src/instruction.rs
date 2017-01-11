
extern crate unescape;

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
