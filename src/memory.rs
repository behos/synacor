pub const MAX_INT: u16 = 32768;
pub const MAX_ADDR: usize = MAX_INT as usize;

pub struct Memory {
    // our reserved space can hold 16bit values in a 15bit (32768) address space
    memory: [u16; MAX_ADDR],
    registers: [u16; 8],
    stack: Vec<u16>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: [0; MAX_ADDR],
            registers: [0; 8],
            stack: vec![],
        }
    }

    pub fn load(&mut self, bytes: Vec<u8>) {
        let mut index = 0;
        let mut buf: u16 = 0;
        for (i, byte) in bytes.iter().enumerate() {
            if i % 2 == 0 {
                buf = *byte as u16;
            } else {
                buf += (*byte as u16) << 8;
                self.memory[index] = buf;
                buf = 0;
                index += 1;
            }
        }
    }

    pub fn at(&self, index: usize) -> u16 {
        // TODO: Check for memory overflow
        self.memory[index]
    }

    pub fn set(&mut self, index: usize, value: u16) {
        self.memory[index] = value;
    }

    pub fn slice(&self, from: usize, to: usize) -> &[u16] {
        &self.memory[from..=to]
    }

    pub fn read_register(&self, index: usize) -> u16 {
        // TODO: Check register overflow
        self.registers[index]
    }

    pub fn set_register(&mut self, index: usize, value: u16) {
        self.registers[index] = value
    }

    pub fn push(&mut self, value: u16) {
        self.stack.push(value)
    }

    pub fn pop(&mut self) -> Option<u16> {
        self.stack.pop()
    }
}
