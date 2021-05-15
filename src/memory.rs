const BIT_15: usize = 32768;

pub struct Memory {
    // our reserved space can hold 16bit values in a 15bit (32768) address space
    pub reserved: [u16; BIT_15],
    pub registers: [u16; 8],
    pub stack: Vec<u16>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            reserved: [0; BIT_15],
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
                self.reserved[index] = buf;
                buf = 0;
                index += 1;
            }
        }
    }
}
