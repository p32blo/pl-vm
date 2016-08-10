
use std::io;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Operand {
    Integer(i32),
    Float(f32),
    Address(usize),
}

impl Operand {
    fn add(n: Self, a: Self) -> Self {
        match (n, a) {
            (Operand::Integer(n), Operand::Address(a)) => Operand::Address((a as i32 + n) as usize),
            _ => panic!("Invalid Operation"),
        }
    }
}


#[derive(Debug,Default)]
struct Machine {
    /// Stack Pointer
    sp: usize,
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

    fn load<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let mut f = try!(File::open(path));
        let mut buffer = String::new();
        try!(f.read_to_string(&mut buffer));
        self.code = buffer.lines().map(|x| x.trim().to_lowercase().to_string()).collect();
        Ok(())
    }

    fn run(&mut self) {
        loop {
            self.run_instruction();
            println!("inst: <{}>\n{:#?}", self.code[self.pc], *self);
        }
    }

    fn run_instruction(&mut self) {
        let (inst, val) = self.get_instruction();

        if inst.contains(":") {
            let pc = self.pc;
            self.label(&inst, pc);
        } else {
            match inst.as_ref() {
                "pushi" => self.pushi(&val.unwrap()),
                "pushn" => self.pushn(&val.unwrap()),
                "pushg" => self.pushg(&val.unwrap()),
                "pushs" => self.pushs(&val.unwrap()),
                "pushgp" => self.pushgp(),
                "start" => self.start(),
                "writes" => self.writes(),
                "read" => self.read(),
                "atoi" => self.atoi(),
                "padd" => self.padd(),
                _ => panic!(format!("Instruction not found: {}", inst)),
            }
        }
        self.pc += 1;
    }

    fn get_instruction(&self) -> (String, Option<String>) {
        let ref inst = self.code[self.pc];

        let find = inst.find(" ");
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
        self.sp += 1;
    }

    fn pushn(&mut self, val: &str) {
        for _ in 0..val.parse().unwrap() {
            self.pushi("0");
        }
    }

    fn push_reg(&mut self, addr: usize) {
        self.stack.push(Operand::Address(addr));
        self.sp += 1;
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
        self.sp += 1;
    }

    fn pushs(&mut self, val: &str) {
        let mut val = val.to_string();
        val.remove(0);
        val.pop().unwrap();
        self.strings.push(val);
        self.stack.push(Operand::Address(self.strings.len() - 1));
    }

    fn start(&self) {}

    fn writes(&mut self) {
        match &self.stack.pop().unwrap() {
            &Operand::Address(addr) => println!("{}", self.strings[addr]),
            _ => panic!("writes: Must be address to write string"),
        }
        self.sp -= 1;
    }

    fn read(&mut self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        self.strings.push(input.trim().to_string());
        self.stack.push(Operand::Address(self.strings.len() - 1));
        self.sp += 1;
    }

    fn atoi(&mut self) {
        let str = match &self.stack.pop().unwrap() {
            &Operand::Address(addr) => self.strings.remove(addr),
            _ => panic!("atoi: Must be address to write string"),
        };
        self.sp -= 1;

        if let Err(_) = str.parse::<usize>() {
            panic!("Not a valid number");
        }
        self.pushi(&str);

    }

    fn label(&mut self, mat: &str, pc: usize) {
        self.labels.insert(mat.to_string(), pc);
    }

    fn padd(&mut self) {
        let n = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(Operand::add(n, a));
        self.sp -= 1;
    }
}


pub fn start<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let mut m = Machine::new();
    try!(m.load(path));
    // println!("{:#?}", m);
    m.run();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
