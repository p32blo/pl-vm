
use std::io;
use std::path::Path;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum Operand {
    Integer(i32),
    Float(f32),
    Address(usize),
}

#[derive(Debug, Default)]
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
    operands: Vec<Operand>,
    /// Call Stack (instruction address, frame pointer)
    stack: (usize, usize),
    /// Code
    code: Vec<String>,
}

impl Machine {
    fn new() -> Self {
        Self::default()
    }

    fn load<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let mut f = try!(File::open(path));
        let mut buffer = String::new();
        try!(f.read_to_string(&mut buffer));
        self.code = buffer.lines().map(|x| x.trim().to_string()).collect::<Vec<String>>();
        Ok(())
    }

    fn run(&self) {
        loop {
            self.run_instruction();
            println!("{:#?}", *self);
        }
    }

    fn run_instruction(&self) {
        let (inst, val) = self.get_instruction();

        match inst {
            _ => panic!(format!("Instruction not found: {}", inst))
        }
    }

    fn get_instruction(&self) -> (String, String) {
        let ref inst = self.code[self.pc];
        let mut split = inst.split_whitespace().map(|x| x.to_string());
        (split.next().unwrap(), split.next().unwrap())
    }
}


pub fn start<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let mut m = Machine::new();
    try!(m.load(path));
    //println!("{:#?}", m);
    m.run();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
