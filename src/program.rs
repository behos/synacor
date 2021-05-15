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
    pub fn new(bytes: Vec<u8>, register8: u16) -> Self {
        let mut memory = Memory::new();
        let input_buffer = InputBuffer {
            buffer: vec![],
            cursor: 0,
        };
        memory.load(bytes);
        memory.set_register(7, register8);
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
        let reg = self.next_register();
        let mut value = self.input_buffer.next();

        if value as char == '!' {
            log::debug!("bypassing register");
            self.memory.set_register(7, 666);
            value = self.input_buffer.next()
        }

        self.memory.set_register(reg, value as u16);
        self.cursor += 1;
        false
    }

    fn run_jump(&mut self) -> bool {
        let arg = self.next_value();
        log::debug!("jump: {}", arg);
        if (arg as usize) < MAX_ADDR {
            self.cursor = arg as usize
        } else {
            panic!("memory address to jump to {:?}", arg)
        }
        false
    }

    fn run_jump_true(&mut self) -> bool {
        let cond = self.next_value();
        let arg = self.next_value();
        log::debug!("jump true: {} {}", cond, arg);
        if cond == 0 {
            self.cursor += 1
        } else {
            self.cursor = arg as usize;
        }
        false
    }

    fn run_jump_false(&mut self) -> bool {
        let cond = self.next_value();
        let arg = self.next_value();
        log::debug!("jump true: {} {}", cond, arg);
        if cond != 0 {
            self.cursor += 1
        } else {
            self.cursor = arg as usize;
        }
        false
    }

    fn run_set(&mut self) -> bool {
        let reg = self.next_register();
        let val = self.next_value();
        log::debug!("set: {} {}", reg, val);
        self.memory.set_register(reg, val);
        self.cursor += 1;
        false
    }

    fn run_push(&mut self) -> bool {
        let val = self.next_value();
        log::debug!("push: {}", val);
        self.memory.push(val);
        self.cursor += 1;
        false
    }

    fn run_pop(&mut self) -> bool {
        let reg = self.next_register();
        let val = self.memory.pop().expect("Should have a value when popping");
        log::debug!("pop: {}", val);
        self.memory.set_register(reg, val);
        self.cursor += 1;
        false
    }

    fn run_add(&mut self) -> bool {
        log::debug!("add");
        self.run_calc(|a, b| a + b)
    }

    fn run_modulo(&mut self) -> bool {
        log::debug!("modulo");
        self.run_calc(|a, b| a % b)
    }

    fn run_multiply(&mut self) -> bool {
        log::debug!("multiply");
        self.run_calc(|a, b| ((a as usize * b as usize) % MAX_INT as usize) as u16)
    }

    fn run_and(&mut self) -> bool {
        log::debug!("and");
        self.run_calc(|a, b| a & b)
    }

    fn run_or(&mut self) -> bool {
        log::debug!("or");
        self.run_calc(|a, b| a | b)
    }

    fn run_calc(&mut self, calc: impl FnOnce(u16, u16) -> u16) -> bool {
        let reg = self.next_register();
        let a = self.next_value();
        let b = self.next_value();
        let res = calc(a, b) % MAX_INT;
        log::debug!("set {} {}", reg, res);
        self.memory.set_register(reg, res);
        self.cursor += 1;
        false
    }

    fn run_not(&mut self) -> bool {
        let reg = self.next_register();
        let val = self.next_value();
        log::debug!("not: {} {}", reg, val);
        self.memory.set_register(reg, !val % MAX_INT);
        self.cursor += 1;
        false
    }

    fn run_equal(&mut self) -> bool {
        log::debug!("equal");
        self.run_cond(|a, b| a == b)
    }

    fn run_greater_than(&mut self) -> bool {
        log::debug!("greater than");
        self.run_cond(|a, b| a > b)
    }

    fn run_cond(&mut self, comp: impl FnOnce(u16, u16) -> bool) -> bool {
        let reg = self.next_register();
        let a = self.next_value();
        let b = self.next_value();
        log::debug!("comp {} {}", a, b);
        let val = if comp(a, b) { 1 } else { 0 };
        log::debug!("set {} {}", reg, val);
        self.memory.set_register(reg, val);
        self.cursor += 1;
        false
    }

    fn run_call(&mut self) -> bool {
        let val = self.next_value();
        log::debug!("call: {}", val);
        self.memory.push((self.cursor + 1) as u16);
        self.cursor = val as usize;
        false
    }

    fn run_return(&mut self) -> bool {
        if let Some(addr) = self.memory.pop() {
            self.cursor = addr as usize;
            log::debug!("return: {}", addr);
            false
        } else {
            log::debug!("return root");
            true
        }
    }

    fn run_read_mem(&mut self) -> bool {
        let reg = self.next_register();
        let addr = self.next_value();
        let val = self.memory.at(addr as usize);
        log::debug!("set: {} {}", reg, val);
        self.memory.set_register(reg, val);
        self.cursor += 1;
        false
    }

    fn run_write_mem(&mut self) -> bool {
        let addr = self.next_value();
        let val = self.next_value();
        log::debug!("set: {} {}", addr, val);
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
