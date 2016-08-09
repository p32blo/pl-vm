
use std::path::Path;


enum Operand {
    Integer(i32),
    Float(f32),
    Address(usize),
}

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
    stack: (usize, usize)
}

pub fn start <P: AsRef<Path>>(path: P) {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
