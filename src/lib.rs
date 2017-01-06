
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate unescape;

use std::io;
use std::io::Write;

use std::fmt;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use errors::*;

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


#[derive(Default)]
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

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "| sp: {:2} |", self.sp())?;
        write!(f, " fp: {:2} |", self.fp)?;
        write!(f, " pc: {:2} |", self.pc)?;
        write!(f, " gp: {:2} |", self.gp)?;

        write!(f, "\nstack:\n")?;
        for val in &self.stack {
            write!(f, "{:?} ", val)?;
        }
        Ok(())
    }
}

impl Machine {
    fn new() -> Self {
        Self::default()
    }

    fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let mut f = File::open(path).chain_err(|| "Failed to open file")?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).chain_err(|| "Unable to Read file")?;
        self.code = buffer.lines()
            .map(|x| Self::remove_comments(&x.trim().to_lowercase()))
            .filter(|x| !x.is_empty())
            .collect();

        for (i, line) in self.code.iter().enumerate() {
            if Self::is_label(line) {
                let mut label = line.to_string();
                label.pop().unwrap();
                self.labels.insert(label, i);
            }
        }

        Ok(())
    }

    fn is_label(line: &str) -> bool {
        if let Some(inst) = line.split_whitespace().next() {
            if inst.contains(':') {
                return true;
            }
        }
        false
    }

    fn sp(&self) -> usize {
        self.stack.len() - 1
    }

    fn stack_pop(&mut self) -> Operand {
        self.stack.pop().expect("Stack is empty")
    }

    fn call_stack_pop(&mut self) -> (usize, usize) {
        self.call_stack.pop().expect("Call stack is empty")
    }

    fn run(&mut self) -> Result<()> {
        // println!("code: {:#?}\nlabels: {:#?}", self.code, self.labels);
        loop {
            match self.run_instruction()? {
                Some(_inst) => {
                    // println!("<{:^8}>\n{:?}", _inst, *self);
                    // io::stdin().read_line(&mut String::new()).unwrap();
                }
                None => break,
            }
        }
        Ok(())
    }

    fn run_instruction(&mut self) -> Result<Option<String>> {
        let (inst, val) = self.get_instruction();

        let val_err = &format!("No value found for '{}' instruction", inst);
        // println!("instr: <{:?}>", (&inst, &val));

        if !Self::is_label(&inst) {
            match inst.as_ref() {
                "pushi" => {
                    self.pushi(val.expect(val_err)
                        .parse()
                        .expect("value is not a positive integer"))
                }
                "pushn" => {
                    self.pushn(val.expect(val_err)
                        .parse()
                        .expect("value is not a positive integer"))
                }
                "pushg" => {
                    self.pushg(val.expect(val_err)
                        .parse()
                        .expect("value is not a positive integer"))
                }
                "pushs" => self.pushs(&val.expect(val_err)),
                "pusha" => self.pusha(&val.expect(val_err)),
                "pushgp" => self.pushgp(),
                "call" => self.call(),
                "return" => self.ret(),
                "start" | "nop" => {}
                "stop" => return Ok(None),
                "loadn" => self.loadn(),
                "writei" => self.writei(),
                "writes" => self.writes(),
                "read" => self.read()?,
                "atoi" => self.atoi()?,
                "padd" => self.padd(),
                "add" => self.add(),
                "mul" => self.mul(),
                "div" => self.div(),
                "mod" => self.module(),
                "storeg" => {
                    self.storeg(val.expect(val_err)
                        .parse()
                        .expect("value is not a positive integer"))
                }
                "storen" => self.storen(),
                "equal" => self.equal(),
                "inf" => self.inf(),
                "infeq" => self.infeq(),
                "sup" => self.sup(),
                "supeq" => self.supeq(),
                "jump" => self.jump(&val.expect(val_err)),
                "jz" => self.jz(&val.expect(val_err)),
                "err" => {
                    let err = Self::remove_quotes(&val.expect(val_err));
                    bail!(format!("End execution with [{}]", err))
                }
                _ => panic!(format!("Instruction not found: {}", inst)),
            }
        }
        self.pc += 1;
        Ok(Some(inst))
    }

    fn remove_comments(inst: &str) -> String {
        match inst.find("//") {
            None => inst.to_string(),
            Some(f) => {
                let (inst, _) = inst.split_at(f);
                inst.trim().to_string()
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


    fn get_instruction(&mut self) -> (String, Option<String>) {
        let mut inst_ref = &self.code[self.pc];

        while inst_ref.is_empty() {
            self.pc += 1;
            inst_ref = &self.code[self.pc];
        }

        let find = inst_ref.find(' ');
        match find {
            None => (inst_ref.to_string(), None),
            Some(f) => {
                let (inst, val) = inst_ref.split_at(f);
                (inst.to_string(), Some(val.trim().to_string()))
            }
        }
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
        let val = Self::remove_quotes(val);

        if let Some(i) = self.strings.iter().position(|x| x == &val) {
            self.stack.push(Operand::Address(i));
        } else {
            self.strings.push(val);
            self.stack.push(Operand::Address(self.strings.len() - 1));
        }
    }

    fn pusha(&mut self, val: &str) {
        let addr = self.labels[val];
        self.stack.push(Operand::Address(addr));
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
        self.pc = self.labels[val];
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


pub fn start<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut m = Machine::new();
    m.load(&path).chain_err(|| format!("Cannot load file '{}'", path.as_ref().display()))?;
    // println!("{:#?}", m);
    m.run()?;
    Ok(())
}
