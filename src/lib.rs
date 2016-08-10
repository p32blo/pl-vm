
use std::io;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug)]
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
            println!("{:#?}", *self);
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
                "pushgp" => self.pushgp(),
                "start" => self.start(),
                "padd" => self.padd(),
                _ => panic!(format!("Instruction not found: {}", inst)),
            }
        }
        self.pc += 1;
    }

    fn get_instruction(&self) -> (String, Option<String>) {
        let ref inst = self.code[self.pc];
        let mut split = inst.split_whitespace().map(|x| x.to_string());
        (split.next().unwrap(), split.next())
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

    fn start(&self) {}

    fn label(&mut self, mat: &str, pc: usize) {
        self.labels.insert(mat.to_string(), pc);
    }

    fn padd(&mut self) {
        Operand::add(self.stack.pop().unwrap(), self.stack.pop().unwrap());
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
