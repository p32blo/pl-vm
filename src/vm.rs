
//! Virtual Machine definition

use std::io;
use std::io::Write;
use std::fmt;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

use errors;
use errors::*;

use instructions::Instruction;
use commands::{Command, Status};

/// The `vm` execution mode
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Debug,
    Running,
}

/// A value that can be in the stack
#[derive(Debug, Clone, Copy)]
enum Operand {
    Integer(i32),
    // Float(f32),
    Address(usize),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::Integer(i) => write!(f, "{:2}i", i),
            Operand::Address(a) => write!(f, "{:2}a", a),
        }
    }
}

impl Operand {
    fn add(n: &Self, a: &Self) -> Self {
        match (n, a) {
            (&Operand::Integer(n), &Operand::Address(a)) => {
                Operand::Address((a as i32 + n) as usize)
            }
            (&Operand::Integer(n), &Operand::Integer(a)) => Operand::Integer(n + a),
            _ => panic!(format!("Operand::add => Invalid Operation: {} + {}", n, a)),
        }
    }

    fn mul(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) => Operand::Integer(n * m),
            _ => panic!(format!("Operand::mul => Invalid Operation: {} * {}", n, m)),
        }
    }

    fn div(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) => Operand::Integer(m / n),
            _ => panic!(format!("Operand::div => Invalid Operation: {} / {}", m, n)),
        }
    }
    fn module(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) => Operand::Integer(m % n),
            _ => panic!(format!("Operand::mod => Invalid Operation: {} % {}", m, n)),
        }
    }

    fn equal(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) if n == m => Operand::Integer(1),
            (&Operand::Integer(..), &Operand::Integer(..)) => Operand::Integer(0),
            _ => panic!(format!("Operand::equal => Invalid Operation: {} == {}", m, n)),
        }
    }

    fn inf(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) if m < n => Operand::Integer(1),
            (&Operand::Integer(..), &Operand::Integer(..)) => Operand::Integer(0),
            _ => panic!(format!("Operand::sup => Invalid Operation: {} < {}", m, n)),
        }
    }

    fn infeq(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) if m <= n => Operand::Integer(1),
            (&Operand::Integer(..), &Operand::Integer(..)) => Operand::Integer(0),
            _ => panic!(format!("Operand::sup => Invalid Operation: {} <= {}", m, n)),
        }
    }

    fn sup(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) if m > n => Operand::Integer(1),
            (&Operand::Integer(..), &Operand::Integer(..)) => Operand::Integer(0),
            _ => panic!(format!("Operand::sup => Invalid Operation: {} > {}", m, n)),
        }
    }

    fn supeq(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) if m >= n => Operand::Integer(1),
            (&Operand::Integer(..), &Operand::Integer(..)) => Operand::Integer(0),
            _ => panic!(format!("Operand::supeq => Invalid Operation: {} >= {}", m, n)),
        }
    }
}

/// The Main struct responsible for the `vm`
#[derive(Default, Clone)]
struct Machine {
    /// Frame Pointer
    fp: usize,
    /// Program Counter
    pc: usize,
    /// Global Variables Base Address
    gp: usize,
    /// Operand Stack
    stack: Vec<Operand>,
    /// Call Stack (instruction address, frame pointer)
    call_stack: Vec<(usize, usize)>,
    /// Code
    code: Vec<Instruction>,
    /// String stack
    strings: Vec<String>,
    /// Label Map
    labels: HashMap<String, usize>,
}

impl Machine {
    fn new() -> Self {
        Self::default()
    }

    fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        // Open file
        let mut f = File::open(path).chain_err(|| "Failed to open file")?;

        // Load file to memory
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).chain_err(|| "Unable to Read file")?;

        // Strip comments and remove empty lines
        let code_labels: Vec<String> = buffer.lines()
            .map(|x| Self::strip_comments(&x.trim().to_lowercase()))
            .filter(|x| !x.is_empty())
            .collect();

        // inserted labels so far
        let mut acc = 0;

        self.labels.clear();
        // insert labels with the correct pointer
        for (i, line) in code_labels.iter().enumerate() {
            if let Some(val) = Self::is_label(line) {
                self.labels.insert(val, i - acc);
                acc += 1;
            }
        }

        self.code.clear();
        // remove labels from code
        for instr in code_labels.iter().filter(|line| Self::is_label(line).is_none()) {
            self.code.push(instr.parse()
                               .chain_err(|| {
                                              format!("Failed to parse '{}' instruction", instr)
                                          })?);
        }
        Ok(())
    }

    fn is_label(line: &str) -> Option<String> {
        let mut inst = line.to_string();
        if inst.ends_with(':') {
            inst.pop().unwrap();
            return Some(inst);
        }
        None
    }

    fn sp(&self) -> usize {
        self.stack.len()
    }

    fn stack_pop(&mut self) -> Operand {
        self.stack.pop().expect("Stack is empty")
    }

    fn call_stack_pop(&mut self) -> (usize, usize) {
        self.call_stack.pop().expect("Call stack is empty")
    }

    fn readline(&self) -> Result<Command> {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).chain_err(|| "Error reading line")?;
        buf.parse()
    }

    fn debug(&mut self, cmd: Command, status: Status) -> Result<Status> {
        match cmd {
            Command::PrintCode => {
                println!("\t/// CODE ///");
                for (i, line) in self.code.iter().enumerate() {
                    for (k, _) in self.labels.iter().filter(|&(_, &v)| v == i) {
                        println!("{}:", k);
                    }
                    println!("  {:>2}|\t{}", i, line);
                }
                Ok(status)
            }
            Command::PrintLabels => {
                println!("Labels:");
                for (k, val) in &self.labels {
                    println!(" | {} => {}", k, val);
                }
                Ok(status)
            }
            Command::PrintRegisters => {
                print!("Registeres: ");
                print!(" sp = {:2} |", self.sp());
                print!(" fp = {:2} |", self.fp);
                print!(" pc = {:2} |", self.pc);
                print!(" gp = {:2} |", self.gp);
                println!();
                Ok(status)
            }
            Command::PrintStack => {
                println!("Stack:");
                print!("--- <- sp");
                if self.fp == self.sp() {
                    print!(" <- fp");
                }
                println!();

                for (i, val) in self.stack
                        .iter()
                        .enumerate()
                        .rev() {
                    print!("{}", val);
                    if self.fp == i {
                        print!(" <- fp");
                    }
                    println!();
                }
                Ok(status)
            }
            Command::Run => {
                Ok(match status {
                       Status::Success => {
                    self.run()?;
                    println!();
                    Status::Exit
                }
                       Status::Exit => status,
                   })
            }
            Command::Next(end) => {
                if let Status::Success = status {
                    let mut bk = self.clone();
                    for _ in 0..end {
                        let instr = bk.get_instruction();
                        println!("\t: {} :", instr);
                        instr.write_ln();
                        if let Status::Exit = bk.run_instruction(&instr)? {
                            break;
                        }
                    }
                }
                Ok(status)
            } 
            Command::Step(end) => {
                if let Status::Success = status {
                    for _ in 0..end {
                        let instr = &self.get_instruction();
                        println!("\t< {} >", instr);
                        let s = self.run_instruction(instr)?;
                        instr.write_ln();
                        if let Status::Exit = s {
                            return Ok(s);
                        }
                    }
                }
                Ok(status)
            }
            Command::Help => {
                Command::help();
                Ok(status)
            }
            Command::Quit => {
                ::std::process::exit(0);
            }
            Command::Empty => Ok(status),
        }
    }



    fn run_debug(&mut self) -> () {
        let mut status = Status::Success;
        loop {
            match status {
                Status::Success => print!("(debug) "),
                Status::Exit => print!("(debug - finished) "),
            }
            io::stdout().flush().expect("Could not flush stdout");

            let cmd = self.readline().unwrap_or_else(|ref e| {
                                                         errors::print_errs(e);
                                                         Command::Empty
                                                     });
            status = self.debug(cmd, status).unwrap_or_else(|ref e| {
                                                                errors::print_errors(e);
                                                                Status::Exit
                                                            });
        }
    }

    fn run(&mut self) -> Result<()> {
        loop {
            let instr = self.get_instruction();
            if let Status::Exit = self.run_instruction(&instr)? {
                break;
            }
        }
        Ok(())
    }

    fn run_instruction(&mut self, inst: &Instruction) -> Result<Status> {
        match *inst {
            Instruction::Pushi(val) => self.pushi(val),
            Instruction::Pushn(val) => self.pushn(val),
            Instruction::Pushg(val) => self.pushg(val),
            Instruction::Pushs(ref val) => self.pushs(val),
            Instruction::Pusha(ref val) => self.pusha(val),
            Instruction::Pushgp => self.pushgp(),
            Instruction::Call => self.call(),
            Instruction::Return => self.ret(),
            Instruction::Start => self.start(),
            Instruction::Nop |
            Instruction::Label(..) | 
            Instruction::Pushf(..) |
            Instruction::Check(..)=> {}
            Instruction::Stop => return Ok(Status::Exit),
            Instruction::Loadn => self.loadn(),
            Instruction::Writei => self.writei(),
            Instruction::Writes => self.writes(),
            Instruction::Read => self.read()?,
            Instruction::Atoi => self.atoi()?,
            Instruction::Padd => self.padd(),
            Instruction::Add => self.add(),
            Instruction::Mul => self.mul(),
            Instruction::Div => self.div(),
            Instruction::Mod => self.module(),
            Instruction::Storeg(val) => self.storeg(val),
            Instruction::Storen => self.storen(),
            Instruction::Equal => self.equal(),
            Instruction::Inf => self.inf(),
            Instruction::Infeq => self.infeq(),
            Instruction::Sup => self.sup(),
            Instruction::Supeq => self.supeq(),
            Instruction::Jump(ref val) => self.jump(val),
            Instruction::Jz(ref val) => self.jz(val),
            Instruction::Err(ref err) => bail!(format!("End execution with [{}]", err)),
        }
        self.pc += 1;

        Ok(Status::Success)
    }

    fn strip_comments(inst: &str) -> String {
        match inst.find("//") {
            None => inst.to_string(),
            Some(f) => {
                let (inst, _) = inst.split_at(f);
                inst.trim().to_string()
            }
        }
    }

    fn get_instruction(&self) -> Instruction {
        self.code[self.pc].clone()
    }

    fn pushi(&mut self, val: i32) {
        self.stack.push(Operand::Integer(val));
    }

    fn pushn(&mut self, val: i32) {
        for _ in 0..val {
            self.pushi(0);
        }
    }

    fn push_reg(&mut self, addr: usize) {
        self.stack.push(Operand::Address(addr));
    }

    fn pushgp(&mut self) {
        let gp = self.gp;
        self.push_reg(gp);
    }

    fn pushg(&mut self, val: usize) {
        let addr = self.gp + val;
        let value = self.stack[addr];

        self.stack.push(value);
    }

    fn pushs(&mut self, val: &str) {
        if let Some(i) = self.strings.iter().position(|x| x == &val) {
            self.stack.push(Operand::Address(i));
        } else {
            self.strings.push(val.to_string());
            self.stack.push(Operand::Address(self.strings.len() - 1));
        }
    }

    fn pusha(&mut self, val: &str) {
        let addr = self.labels[val] - 1;
        self.stack.push(Operand::Address(addr));
    }

    fn start(&mut self) {
        self.fp = self.sp();
    }

    fn loadn(&mut self) {
        let n = self.stack_pop();
        let a = self.stack_pop();

        if let Operand::Address(addr) = Operand::add(&n, &a) {
            let v = self.stack[addr];
            self.stack.push(v);
        } else {
            panic!("loadn: Not an Address");
        }
    }

    fn writei(&mut self) {
        let val = self.stack_pop();
        if let Operand::Integer(i) = val {
            print!("{}", i);
            io::stdout().flush().expect("Could not flush stdout");
        } else {
            panic!("writei: Not an Integer");
        }

    }

    fn writes(&mut self) {
        match self.stack_pop() {
            Operand::Address(addr) => {
                print!("{}", self.strings[addr]);
                io::stdout().flush().expect("Could not flush stdout");
            }
            _ => panic!("writes: Must be address to write string"),
        }
    }

    fn read(&mut self) -> Result<()> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).chain_err(|| "Failed to read line from stdin")?;
        self.strings.push(input.trim().to_string());
        self.stack.push(Operand::Address(self.strings.len() - 1));
        Ok(())
    }

    fn atoi(&mut self) -> Result<()> {
        let str = match self.stack_pop() {
            Operand::Address(addr) => self.strings.remove(addr),
            _ => bail!("atoi: Must be address to write string"),
        };

        match str.parse() {
            Ok(val) => Ok(self.pushi(val)),
            Err(_) => bail!("Value is not a valid Integer"),
        }
    }

    fn storeg(&mut self, n: usize) {
        let val = self.stack_pop();
        self.stack[self.gp + n] = val;
    }

    fn storen(&mut self) {
        let v = self.stack_pop();
        let n = self.stack_pop();
        let a = self.stack_pop();

        match Operand::add(&n, &a) {
            Operand::Address(addr) => self.stack[addr] = v,
            _ => panic!("storen: Not an Address"),
        }
    }

    fn call(&mut self) {
        if let Operand::Address(addr) = self.stack_pop() {
            self.call_stack.push((self.pc, self.fp));

            self.fp = self.sp();
            self.pc = addr;
        } else {
            panic!("call: Not an Address");
        }
    }

    fn ret(&mut self) {
        let (pc, fp) = self.call_stack_pop();
        self.pc = pc;
        self.fp = fp;
    }


    fn binary_op<F: FnOnce(&Operand, &Operand) -> Operand>(&mut self, op: F) {
        let n = self.stack_pop();
        let m = self.stack_pop();

        let val = op(&n, &m);

        self.stack.push(val);
    }

    fn padd(&mut self) {
        self.binary_op(Operand::add)
    }

    fn add(&mut self) {
        self.binary_op(Operand::add)
    }

    fn mul(&mut self) {
        self.binary_op(Operand::mul)
    }

    fn div(&mut self) {
        self.binary_op(Operand::div)
    }

    fn module(&mut self) {
        self.binary_op(Operand::module)
    }

    fn equal(&mut self) {
        self.binary_op(Operand::equal)
    }

    fn inf(&mut self) {
        self.binary_op(Operand::inf)
    }

    fn infeq(&mut self) {
        self.binary_op(Operand::infeq)
    }

    fn sup(&mut self) {
        self.binary_op(Operand::sup)
    }

    fn supeq(&mut self) {
        self.binary_op(Operand::supeq)
    }

    fn jump(&mut self, val: &str) {
        self.pc = self.labels[val] - 1;
    }

    fn jz(&mut self, val: &str) {
        let eq = self.stack_pop();

        match eq {
            Operand::Integer(0) => self.jump(val),
            Operand::Integer(1) => {}
            _ => panic!("jz: Not an Integer(0|1)"),
        }
    }
}

/// `vm` entry point
pub fn start<P: AsRef<Path>>(path: P, mode: Mode) -> Result<()> {
    let mut m = Machine::new();
    m.load(&path).chain_err(|| format!("Cannot load file '{}'", path.as_ref().display()))?;
    // println!("{:#?}", m);
    match mode {
        Mode::Running => m.run()?,
        Mode::Debug => m.run_debug(),
    }

    Ok(())
}
