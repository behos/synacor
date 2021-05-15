enum Operation {
    Halt,
    Out,
    Noop,
}

impl From<u16> for Operation {
    fn from(opcode: u16) -> Self {
        match opcode {
            0 => Operation::Halt,
            19 => Operation::Out,
            21 => Operation::Noop,
            a if a > 21 => panic!("Invalid op code {}", opcode),
            _ => unimplemented!("Missing implementation for opcode {}", opcode),
        }
    }
}
