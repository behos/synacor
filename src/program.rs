use crate::{
    memory::{Memory, MAX_ADDR, MAX_INT},
    operations::Operation,
    values::Value,
};
use std::io;

struct InputBuffer {
    buffer: Vec<u8>,
    cursor: usize,
}

impl InputBuffer {
    fn next(&mut self) -> u8 {
        if self.cursor == self.buffer.len() {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("error reading input");
            self.buffer = input.into_bytes();
            self.cursor = 0;
        }
        let c = self.buffer[self.cursor];
        self.cursor += 1;
        c
    }
}

pub struct Program {
    cursor: usize,
    memory: Memory,
    input_buffer: InputBuffer,
}

impl Program {
    pub fn new(bytes: Vec<u8>) -> Self {
        let mut memory = Memory::new();
        let input_buffer = InputBuffer {
            buffer: vec![],
            cursor: 0,
        };
        memory.load(bytes);
        Self {
            cursor: 0,
            memory,
            input_buffer,
        }
    }

    fn step(&mut self) -> bool {
        let opcode = Operation::from(self.memory.at(self.cursor));
        match opcode {
            Operation::Out => self.run_out(),
            Operation::Input => self.run_input(),
            Operation::Equal => self.run_equal(),
            Operation::GreaterThan => self.run_greater_than(),
            Operation::And => self.run_and(),
            Operation::Or => self.run_or(),
            Operation::Not => self.run_not(),
            Operation::Call => self.run_call(),
            Operation::Return => self.run_return(),
            Operation::ReadMem => self.run_read_mem(),
            Operation::WriteMem => self.run_write_mem(),
            Operation::Set => self.run_set(),
            Operation::Push => self.run_push(),
            Operation::Pop => self.run_pop(),
            Operation::Jump => self.run_jump(),
            Operation::JumpTrue => self.run_jump_true(),
            Operation::JumpFalse => self.run_jump_false(),
            Operation::Add => self.run_add(),
            Operation::Modulo => self.run_modulo(),
            Operation::Multiply => self.run_multiply(),
            Operation::Halt => self.run_halt(),
            Operation::Noop => self.run_noop(),
        }
    }

    pub fn execute(&mut self) {
        while !self.step() {}
        log::info!("Done")
    }

    fn run_noop(&mut self) -> bool {
        log::debug!("Running noop");
        self.cursor += 1;
        false
    }

    fn run_halt(&mut self) -> bool {
        log::debug!("Running halt");
        true
    }

    fn run_out(&mut self) -> bool {
        log::debug!("Running out");
        let arg = self.next_value();
        if arg > 127 {
            panic!("Invalid character byte {:?}", arg)
        } else {
            print!("{}", arg as u8 as char)
        }
        self.cursor += 1;
        false
    }

    fn run_input(&mut self) -> bool {
        log::debug!("Running input");
        let reg = self.next_register();
        let value = self.input_buffer.next();
        log::debug!("storing {} to {}", value, reg);
        self.memory.set_register(reg, value as u16);
        self.cursor += 1;
        false
    }

    fn run_jump(&mut self) -> bool {
        log::debug!("Running jump");
        let arg = self.next_value();
        if (arg as usize) < MAX_ADDR {
            self.cursor = arg as usize
        } else {
            panic!("Invalid memory address to jump to {:?}", arg)
        }
        false
    }

    fn run_jump_true(&mut self) -> bool {
        log::debug!("Running jump true");
        log::debug!("{:?}", self.memory.slice(self.cursor, self.cursor + 2),);
        let cond = self.next_value();
        let arg = self.next_value();
        if cond == 0 {
            self.cursor += 1
        } else {
            self.cursor = arg as usize;
        }
        false
    }

    fn run_jump_false(&mut self) -> bool {
        log::debug!("Running jump false");
        let cond = self.next_value();
        let arg = self.next_value();
        if cond != 0 {
            self.cursor += 1
        } else {
            self.cursor = arg as usize;
        }
        false
    }

    fn run_set(&mut self) -> bool {
        log::debug!("Running set");
        log::debug!("{:?}", self.memory.slice(self.cursor, self.cursor + 2),);
        let reg = self.next_register();
        let val = self.next_value();
        self.memory.set_register(reg, val);
        self.cursor += 1;
        false
    }

    fn run_push(&mut self) -> bool {
        log::debug!("Running push");
        log::debug!("{:?}", self.memory.slice(self.cursor, self.cursor + 1),);
        let val = self.next_value();
        self.memory.push(val);
        self.cursor += 1;
        false
    }

    fn run_pop(&mut self) -> bool {
        log::debug!("Running pop");
        log::debug!("{:?}", self.memory.slice(self.cursor, self.cursor + 1),);
        let reg = self.next_register();
        let val = self.memory.pop().expect("Should have a value when popping");
        self.memory.set_register(reg, val);
        self.cursor += 1;
        false
    }

    fn run_add(&mut self) -> bool {
        log::debug!("Running add");
        self.run_calc(|a, b| a + b)
    }

    fn run_modulo(&mut self) -> bool {
        log::debug!("Running modulo");
        self.run_calc(|a, b| a % b)
    }

    fn run_multiply(&mut self) -> bool {
        log::debug!("Running multiply");
        self.run_calc(|a, b| ((a as usize * b as usize) % MAX_INT as usize) as u16)
    }

    fn run_and(&mut self) -> bool {
        log::debug!("Running and");
        self.run_calc(|a, b| a & b)
    }

    fn run_or(&mut self) -> bool {
        log::debug!("Running or");
        self.run_calc(|a, b| a | b)
    }

    fn run_calc(&mut self, calc: impl FnOnce(u16, u16) -> u16) -> bool {
        log::debug!("{:?}", self.memory.slice(self.cursor, self.cursor + 3));
        let reg = self.next_register();
        let a = self.next_value();
        let b = self.next_value();
        self.memory.set_register(reg, calc(a, b) % MAX_INT);
        self.cursor += 1;
        false
    }

    fn run_not(&mut self) -> bool {
        log::debug!("Running not");
        log::debug!("{:?}", self.memory.slice(self.cursor, self.cursor + 2));
        let reg = self.next_register();
        let a = self.next_value();
        self.memory.set_register(reg, !a % MAX_INT);
        self.cursor += 1;
        false
    }

    fn run_equal(&mut self) -> bool {
        log::debug!("Running equal");
        self.run_cond(|a, b| a == b)
    }

    fn run_greater_than(&mut self) -> bool {
        log::debug!("Running greater than");
        self.run_cond(|a, b| a > b)
    }

    fn run_cond(&mut self, comp: impl FnOnce(u16, u16) -> bool) -> bool {
        log::debug!("{:?}", self.memory.slice(self.cursor, self.cursor + 3));
        let reg = self.next_register();
        let a = self.next_value();
        let b = self.next_value();
        self.memory
            .set_register(reg, if comp(a, b) { 1 } else { 0 });
        self.cursor += 1;
        false
    }

    fn run_call(&mut self) -> bool {
        log::debug!("Running call");
        log::debug!("{:?}", self.memory.slice(self.cursor, self.cursor + 1));
        let val = self.next_value();
        self.memory.push((self.cursor + 1) as u16);
        self.cursor = val as usize;
        false
    }

    fn run_return(&mut self) -> bool {
        log::debug!("Running return");
        if let Some(addr) = self.memory.pop() {
            self.cursor = addr as usize;
            false
        } else {
            true
        }
    }

    fn run_read_mem(&mut self) -> bool {
        log::debug!("Running read memory");
        log::debug!("{:?}", self.memory.slice(self.cursor, self.cursor + 2));
        let reg = self.next_register();
        let addr = self.next_value();
        let val = self.memory.at(addr as usize);
        self.memory.set_register(reg, val);
        self.cursor += 1;
        false
    }

    fn run_write_mem(&mut self) -> bool {
        log::debug!("Running write memory");
        log::debug!("{:?}", self.memory.slice(self.cursor, self.cursor + 2));
        let addr = self.next_value();
        let val = self.next_value();
        self.memory.set(addr as usize, val);
        self.cursor += 1;
        false
    }

    /// Retrives the next value from memory and steps the cursor
    fn next_value(&mut self) -> u16 {
        self.cursor += 1;
        match Value::from(self.memory.at(self.cursor)) {
            Value::Register(r) => self.memory.read_register(r as usize),
            Value::Literal(num) => num,
        }
    }

    fn next_register(&mut self) -> usize {
        self.cursor += 1;
        match Value::from(self.memory.at(self.cursor)) {
            Value::Register(r) => r as usize,
            Value::Literal(num) => panic!("expected register but got literal {}", num),
        }
    }
}
