

extern crate unescape;

use std::io;
use std::io::Write;
use std::fmt;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

use std::str::FromStr;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use self::errors::*;

enum Status {
    Success,
    Exit,
}

pub enum Mode {
    Debug,
    Running,
}

enum Command {
    Run,
    Step(usize),
    Next(usize),
    PrintRegisters,
    PrintStack,
    PrintCode,
    PrintLabels,
    Help,
    Quit,
    Empty,
}

impl Command {
    fn help() {
        let help = [("r, run", "Continue the execution"),
                    ("s, step [NUMBER]", "Step by NUMBER instructions. NUMBER defaults to 1"),
                    ("n, next [NUMBER]", "Show NUMBER instructions. NUMBER defaults to 1"),
                    ("reg, registers", "Print the current value for the registers"),
                    ("st, stack", "Print the current state of the stack"),
                    ("c, code", "Print the code that is beeing run"),
                    ("l, labels", "Print all labels found in the code"),
                    ("h, help", "Print this message"),
                    ("q, quit", "Exit from the debugger")];

        println!();
        println!("COMMANDS:");
        for &(cmd, msg) in &help {
            println!("\t{:20}{}", cmd, msg);
        }
        println!("")
    }
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Command> {
        let mut args = s.split_whitespace();
        if let Some(cmd) = args.next() {
            let res = match cmd.to_lowercase().as_ref() {
                "reg" | "registers" => Ok(Command::PrintRegisters),
                "st" | "stack" => Ok(Command::PrintStack),
                "l" | "labels" => Ok(Command::PrintLabels),
                "c" | "code" => Ok(Command::PrintCode),
                "h" | "help" => Ok(Command::Help),
                "q" | "quit" => Ok(Command::Quit),
                "r" | "run" => Ok(Command::Run),
                "n" | "next" => {
                    Ok(Command::Next(args.next()
                        .unwrap_or("1")
                        .parse()
                        .chain_err(|| "Not a valid argument")?))
                }
                "s" | "step" => {
                    Ok(Command::Step(args.next()
                        .unwrap_or("1")
                        .parse()
                        .chain_err(|| "Not a valid argument")?))
                }
                _ => Err("Command not Found. Try 'help' to find valid commands".into()),
            };

            match args.next() {
                None => res,
                Some(_) => res.and(Err("Invalid argument. See 'help' for usage".into())),
            }

        } else {
            Ok(Command::Empty)
        }
    }
}

#[derive(Clone, Copy)]
enum Operand {
    Integer(i32),
    // Float(f32),
    Address(usize),
}

impl fmt::Debug for Operand {
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
            _ => panic!(format!("Operand::add => Invalid Operation: {:?} + {:?}", n, a)),
        }
    }

    fn mul(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) => Operand::Integer(n * m),
            _ => panic!(format!("Operand::mul => Invalid Operation: {:?} * {:?}", n, m)),
        }
    }

    fn div(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) => Operand::Integer(m / n),
            _ => panic!(format!("Operand::div => Invalid Operation: {:?} / {:?}", m, n)),
        }
    }
    fn module(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) => Operand::Integer(m % n),
            _ => panic!(format!("Operand::mod => Invalid Operation: {:?} % {:?}", m, n)),
        }
    }

    fn equal(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) if n == m => Operand::Integer(1),
            (&Operand::Integer(..), &Operand::Integer(..)) => Operand::Integer(0),
            _ => panic!(format!("Operand::equal => Invalid Operation: {:?} == {:?}", m, n)),
        }
    }

    fn inf(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) if m < n => Operand::Integer(1),
            (&Operand::Integer(..), &Operand::Integer(..)) => Operand::Integer(0),
            _ => panic!(format!("Operand::sup => Invalid Operation: {:?} < {:?}", m, n)),
        }
    }

    fn infeq(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) if m <= n => Operand::Integer(1),
            (&Operand::Integer(..), &Operand::Integer(..)) => Operand::Integer(0),
            _ => panic!(format!("Operand::sup => Invalid Operation: {:?} <= {:?}", m, n)),
        }
    }

    fn sup(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) if m > n => Operand::Integer(1),
            (&Operand::Integer(..), &Operand::Integer(..)) => Operand::Integer(0),
            _ => panic!(format!("Operand::sup => Invalid Operation: {:?} > {:?}", m, n)),
        }
    }

    fn supeq(n: &Self, m: &Self) -> Self {
        match (n, m) {
            (&Operand::Integer(n), &Operand::Integer(m)) if m >= n => Operand::Integer(1),
            (&Operand::Integer(..), &Operand::Integer(..)) => Operand::Integer(0),
            _ => panic!(format!("Operand::supeq => Invalid Operation: {:?} >= {:?}", m, n)),
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
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

    fn write_ln(&self) {
        match *self {
            Instruction::Writei | Instruction::Writes => println!(),
            _ => {}
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(instr: &str) -> Result<Instruction> {
        let (inst, val) = Self::decode(instr);
        let val_err = &format!("No value found for '{}' instruction", inst);
        Ok(match inst.as_ref() {
            "pushi" => {
                Instruction::Pushi(val.expect(val_err)
                    .parse()
                    .expect("value is not a positive integer"))
            }
            "pushn" => {
                Instruction::Pushn(val.expect(val_err)
                    .parse()
                    .expect("value is not a positive integer"))
            }
            "pushg" => {
                Instruction::Pushg(val.expect(val_err)
                    .parse()
                    .expect("value is not a positive integer"))
            }
            "pushs" => Instruction::Pushs(Self::remove_quotes(&val.expect(val_err))),
            "pusha" => Instruction::Pusha(val.expect(val_err)),
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
            "storeg" => {
                Instruction::Storeg(val.expect(val_err)
                    .parse()
                    .expect("value is not a positive integer"))
            }
            "storen" => Instruction::Storen,
            "equal" => Instruction::Equal,
            "inf" => Instruction::Inf,
            "infeq" => Instruction::Infeq,
            "sup" => Instruction::Sup,
            "supeq" => Instruction::Supeq,
            "jump" => Instruction::Jump(val.expect(val_err)),
            "jz" => Instruction::Jz(val.expect(val_err)),
            "err" => Instruction::Err(Self::remove_quotes(&val.expect(val_err))),
            _ => panic!(format!("Instruction not found: {}", inst)),
        })
    }
}



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
    code: Vec<String>,
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

        // insert labels with the correct pointer
        for (i, line) in code_labels.iter().enumerate() {
            if let Some(val) = Self::is_label(&line) {
                self.labels.insert(val, i - acc);
                acc += 1;
            }
        }

        // remove labels from code
        self.code = code_labels.iter()
            .filter(|line| Self::is_label(&line).is_none())
            .cloned()
            .collect();

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

    fn run_debug(&mut self) -> Result<()> {
        loop {
            print!("(debug) ");
            io::stdout().flush().expect("Could not flush stdout");
            match self.readline() {
                Ok(cmd) => {
                    match cmd {
                        Command::PrintCode => {
                            println!("\t/// CODE ///");
                            for (i, line) in self.code.iter().enumerate() {
                                for (k, _) in self.labels.iter().filter(|&(_, &v)| v == i) {
                                    println!("{}:", k);
                                }
                                println!("  {:>2}|\t{}", i, line);
                            }
                        }
                        Command::PrintLabels => {
                            println!("Labels:");
                            for (k, val) in &self.labels {
                                println!(" | {} => {}", k, val);
                            }
                        }
                        Command::PrintRegisters => {
                            print!("Registeres: ");
                            print!(" sp = {:2} |", self.sp());
                            print!(" fp = {:2} |", self.fp);
                            print!(" pc = {:2} |", self.pc);
                            print!(" gp = {:2} |", self.gp);
                            println!();
                        }
                        Command::PrintStack => {
                            println!("Stack:");
                            print!("--- <- sp");
                            if self.fp == self.sp() {
                                print!(" <- fp");
                            }
                            println!();

                            for (i, val) in self.stack.iter().enumerate().rev() {
                                print!("{:?}", val);
                                if self.fp == i {
                                    print!(" <- fp");
                                }
                                println!();
                            }
                        }
                        Command::Run => {
                            return self.run();
                        }
                        Command::Next(end) => {
                            let mut bk = self.clone();
                            for _ in 0..end {
                                let instr = bk.get_instruction().parse()?;
                                println!("\t: {:?} :", instr);
                                if let Status::Exit = bk.run_instruction(&instr)? {
                                    break;
                                }
                                instr.write_ln();
                            }
                        }
                        Command::Step(end) => {
                            for _ in 0..end {
                                let instr = self.get_instruction().parse()?;
                                println!("\t< {:?} >", instr);
                                if let Status::Exit = self.run_instruction(&instr)? {
                                    break;
                                }
                                instr.write_ln();
                            }
                        }
                        Command::Help => {
                            Command::help();
                        }
                        Command::Quit => {
                            ::std::process::exit(0);
                        }
                        Command::Empty => {}
                    }
                }
                Err(ref e) => {
                    print!("\t{}. ", e);
                    for e in e.iter().skip(1) {
                        print!("{}. ", e);
                    }
                    println!();
                }
            }
        }
    }

    fn run(&mut self) -> Result<()> {
        loop {
            let instr = self.get_instruction().parse()?;
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
            Instruction::Nop => {}
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

    fn get_instruction(&mut self) -> &str {
        &self.code[self.pc]
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
            print!("{:?}", i);
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
            _ => panic!("atoi: Must be address to write string"),
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

pub fn start<P: AsRef<Path>>(path: P, mode: Mode) -> Result<()> {
    let mut m = Machine::new();
    m.load(&path).chain_err(|| format!("Cannot load file '{}'", path.as_ref().display()))?;
    // println!("{:#?}", m);
    match mode {
        Mode::Running => m.run()?,
        Mode::Debug => m.run_debug()?,
    }

    Ok(())
}
