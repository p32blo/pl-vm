
extern crate unescape;
extern crate ansi_term;

use ansi_term::Colour::Red;

use std::io;
use std::io::Write;

use std::fmt;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

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
            (&Operand::Integer(n), &Operand::Address(a)) => Operand::Address((a as i32 + n) as usize),
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

    fn load<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let mut f = File::open(path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
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

    fn run(&mut self) -> io::Result<()> {
        // println!("code: {:#?}\nlabels: {:#?}", self.code, self.labels);
        loop {
            let _inst = self.run_instruction()?;
            // println!("<{:^8}>\n{:?}", _inst, *self);
            // io::stdin().read_line(&mut String::new()).unwrap();
        }
    }

    fn run_instruction(&mut self) -> io::Result<String> {
        let (inst, val) = self.get_instruction();

        // println!("instr: <{:?}>", (&inst, &val));

        if !Self::is_label(&inst) {
            match inst.as_ref() {
                "pushi" => self.pushi(&val.unwrap()),
                "pushn" => self.pushn(&val.unwrap()),
                "pushg" => self.pushg(&val.unwrap()),
                "pushs" => self.pushs(&val.unwrap()),
                "pusha" => self.pusha(&val.unwrap()),
                "pushgp" => self.pushgp(),
                "call" => self.call(),
                "return" => self.ret(),
                "start" | "nop" => {}
                "stop" => return Err(io::Error::new(io::ErrorKind::Other, "End execution")),
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
                "storeg" => self.storeg(&val.unwrap()),
                "storen" => self.storen(),
                "equal" => self.equal(),
                "inf" => self.inf(),
                "infeq" => self.infeq(),
                "sup" => self.sup(),
                "supeq" => self.supeq(),
                "jump" => self.jump(&val.unwrap()),
                "jz" => self.jz(&val.unwrap()),
                "err" => {
                    let err = Red.paint(Self::remove_quotes(&val.unwrap()));
                    println!("");
                    print!("{}", err);
                    io::stdout().flush().expect("Could not flush stdout");
                    return Err(io::Error::new(io::ErrorKind::Other,
                                              format!("End execution: {}", err)));
                }
                _ => panic!(format!("Instruction not found: {}", inst)),
            }
        }
        self.pc += 1;
        Ok(inst)
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
        let mut inst = &self.code[self.pc];

        while inst.is_empty() {
            self.pc += 1;
            inst = &self.code[self.pc];
        }

        let find = inst.find(' ');
        match find {
            None => (inst.to_string(), None),
            Some(f) => {
                let (inst, val) = inst.split_at(f);
                (inst.to_string(), Some(val.trim().to_string()))
            }
        }
    }

    fn pushi(&mut self, val: &str) {
        self.stack.push(Operand::Integer(val.parse().unwrap()));
    }

    fn pushn(&mut self, val: &str) {
        for _ in 0..val.parse().unwrap() {
            self.pushi("0");
        }
    }

    fn push_reg(&mut self, addr: usize) {
        self.stack.push(Operand::Address(addr));
    }

    fn pushgp(&mut self) {
        let gp = self.gp;
        self.push_reg(gp);
    }

    fn pushg(&mut self, val: &str) {
        let val: usize = val.parse().unwrap();
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
        let n = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        if let Operand::Address(addr) = Operand::add(&n, &a) {
            let v = self.stack[addr];
            self.stack.push(v);
        } else {
            panic!("loadn: Not an Address");
        }
    }

    fn writei(&mut self) {
        let val = self.stack.pop().unwrap();
        if let Operand::Integer(i) = val {
            print!("{:?}", i);
            io::stdout().flush().expect("Could not flush stdout");
        } else {
            panic!("writei: Not an Integer");
        }

    }

    fn writes(&mut self) {
        match self.stack.pop().unwrap() {
            Operand::Address(addr) => {
                print!("{}", self.strings[addr]);
                io::stdout().flush().expect("Could not flush stdout");
            }
            _ => panic!("writes: Must be address to write string"),
        }
    }

    fn read(&mut self) -> io::Result<()> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        self.strings.push(input.trim().to_string());
        self.stack.push(Operand::Address(self.strings.len() - 1));
        Ok(())
    }

    fn atoi(&mut self) -> io::Result<()> {
        let str = match self.stack.pop().unwrap() {
            Operand::Address(addr) => self.strings.remove(addr),
            _ => panic!("atoi: Must be address to write string"),
        };

        if let Err(_) = str.parse::<usize>() {
            Err(io::Error::new(io::ErrorKind::Other,
                               format!("End execution: {}", Red.paint("Not a valid number"))))
        } else {
            Ok(self.pushi(&str))
        }
    }

    fn storeg(&mut self, num: &str) {
        let n: usize = num.parse().unwrap();
        let val = self.stack.pop().unwrap();

        self.stack[self.gp + n] = val;
    }

    fn storen(&mut self) {
        let v = self.stack.pop().unwrap();
        let n = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        if let Operand::Address(addr) = Operand::add(&n, &a) {
            self.stack[addr] = v;
        } else {
            panic!("storen: Not an Address");
        }
    }

    fn call(&mut self) {
        if let Operand::Address(addr) = self.stack.pop().unwrap() {
            self.call_stack.push((self.pc, self.fp));

            self.fp = self.sp();
            self.pc = addr;
        } else {
            panic!("call: Not an Address");
        }
    }

    fn ret(&mut self) {
        let (pc, fp) = self.call_stack.pop().unwrap();
        self.pc = pc;
        self.fp = fp;
    }


    fn binary_op<F: FnOnce(&Operand, &Operand) -> Operand>(&mut self, op: F) {
        let n = self.stack.pop().unwrap();
        let m = self.stack.pop().unwrap();

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
        let eq = self.stack.pop().unwrap();

        match eq {
            Operand::Integer(0) => self.jump(val),
            Operand::Integer(1) => {}
            _ => panic!("jz: Not an Integer(0|1)"),
        }
    }
}


pub fn start<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let mut m = Machine::new();
    m.load(path)?;
    // println!("{:#?}", m);
    m.run()?;
    Ok(())
}
