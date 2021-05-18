use crate::{
    memory::{Memory, MAX_ADDR, MAX_INT},
    operations::Operation,
    values::{Register, Value},
};

use anyhow::{bail, Result};
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
        let operation = self.next_operation().expect("should be a valid operation");
        match operation {
            Operation::Halt => self.run_halt(),
            Operation::Set(r, v) => self.run_set(r, v),
            Operation::Push(v) => self.run_push(v),
            Operation::Pop(r) => self.run_pop(r),
            Operation::Equal(r, v1, v2) => self.run_equal(r, v1, v2),
            Operation::GreaterThan(r, v1, v2) => self.run_greater_than(r, v1, v2),
            Operation::Jump(v) => self.run_jump(v),
            Operation::JumpTrue(v1, v2) => self.run_jump_true(v1, v2),
            Operation::JumpFalse(v1, v2) => self.run_jump_false(v1, v2),
            Operation::Add(r, v1, v2) => self.run_add(r, v1, v2),
            Operation::Multiply(r, v1, v2) => self.run_multiply(r, v1, v2),
            Operation::Modulo(r, v1, v2) => self.run_modulo(r, v1, v2),
            Operation::And(r, v1, v2) => self.run_and(r, v1, v2),
            Operation::Or(r, v1, v2) => self.run_or(r, v1, v2),
            Operation::Not(r, v) => self.run_not(r, v),
            Operation::ReadMem(r, v) => self.run_read_mem(r, v),
            Operation::WriteMem(v1, v2) => self.run_write_mem(v1, v2),
            Operation::Call(v) => self.run_call(v),
            Operation::Return => self.run_return(),
            Operation::Out(v) => self.run_out(v),
            Operation::Input(r) => self.run_input(r),
            Operation::Noop => self.run_noop(),
        }
    }

    fn next_operation(&mut self) -> Result<Operation> {
        let pos = self.cursor;
        let opcode = self.memory.at(self.cursor);
        let operation = match opcode {
            0 => Operation::Halt,
            1 => Operation::Set(self.next_register()?, self.next_value()),
            2 => Operation::Push(self.next_value()),
            3 => Operation::Pop(self.next_register()?),
            4 => Operation::Equal(self.next_register()?, self.next_value(), self.next_value()),
            5 => {
                Operation::GreaterThan(self.next_register()?, self.next_value(), self.next_value())
            }
            6 => Operation::Jump(self.next_value()),
            7 => Operation::JumpTrue(self.next_value(), self.next_value()),
            8 => Operation::JumpFalse(self.next_value(), self.next_value()),
            9 => Operation::Add(self.next_register()?, self.next_value(), self.next_value()),
            10 => Operation::Multiply(self.next_register()?, self.next_value(), self.next_value()),
            11 => Operation::Modulo(self.next_register()?, self.next_value(), self.next_value()),
            12 => Operation::And(self.next_register()?, self.next_value(), self.next_value()),
            13 => Operation::Or(self.next_register()?, self.next_value(), self.next_value()),
            14 => Operation::Not(self.next_register()?, self.next_value()),
            15 => Operation::ReadMem(self.next_register()?, self.next_value()),
            16 => Operation::WriteMem(self.next_value(), self.next_value()),
            17 => Operation::Call(self.next_value()),
            18 => Operation::Return,
            19 => Operation::Out(self.next_value()),
            20 => Operation::Input(self.next_register()?),
            21 => Operation::Noop,
            a if a > 21 => bail!("invalid opcode {}", a),
            _ => unimplemented!("Missing implementation for opcode {}", opcode),
        };
        log::debug!("[{:05}] {:?}", pos, operation);
        Ok(operation)
    }

    pub fn execute(&mut self) {
        while !self.step() {}
        log::info!("Done")
    }

    fn run_noop(&mut self) -> bool {
        self.cursor += 1;
        false
    }

    fn run_halt(&mut self) -> bool {
        true
    }

    fn run_out(&mut self, arg: Value) -> bool {
        let arg = self.val(arg);
        if arg > 127 {
            panic!("Invalid character byte {:?}", arg)
        } else {
            print!("{}", arg as u8 as char)
        }
        self.cursor += 1;
        false
    }

    fn run_input(&mut self, reg: Register) -> bool {
        let mut value = self.input_buffer.next();

        if value as char == '!' {
            log::debug!("overriding register");
            self.memory.set_register(7, 25734);
            value = self.input_buffer.next()
        }

        self.memory.set_register(reg.into(), value as u16);
        self.cursor += 1;
        false
    }

    fn run_jump(&mut self, arg: Value) -> bool {
        let arg = self.val(arg);
        if (arg as usize) < MAX_ADDR {
            self.cursor = arg as usize
        } else {
            panic!("memory address to jump to {:?}", arg)
        }
        false
    }

    fn run_jump_true(&mut self, cond: Value, arg: Value) -> bool {
        let cond = self.val(cond);
        let arg = self.val(arg);

        if cond == 0 {
            self.cursor += 1
        } else {
            self.cursor = arg as usize;
        }
        false
    }

    fn run_jump_false(&mut self, cond: Value, arg: Value) -> bool {
        let cond = self.val(cond);
        let arg = self.val(arg);
        if cond != 0 {
            self.cursor += 1
        } else {
            self.cursor = arg as usize;
        }
        false
    }

    fn run_set(&mut self, reg: Register, val: Value) -> bool {
        let val = self.val(val);
        self.memory.set_register(reg.into(), val);
        self.cursor += 1;
        false
    }

    fn run_push(&mut self, val: Value) -> bool {
        let val = self.val(val);
        self.memory.push(val);
        self.cursor += 1;
        false
    }

    fn run_pop(&mut self, reg: Register) -> bool {
        let val = self.memory.pop().expect("Should have a value when popping");
        self.memory.set_register(reg.into(), val);
        self.cursor += 1;
        false
    }

    fn run_add(&mut self, reg: Register, v1: Value, v2: Value) -> bool {
        self.run_calc(reg, v1, v2, |a, b| a + b)
    }

    fn run_modulo(&mut self, reg: Register, v1: Value, v2: Value) -> bool {
        self.run_calc(reg, v1, v2, |a, b| a % b)
    }

    fn run_multiply(&mut self, reg: Register, v1: Value, v2: Value) -> bool {
        self.run_calc(reg, v1, v2, |a, b| {
            ((a as usize * b as usize) % MAX_INT as usize) as u16
        })
    }

    fn run_and(&mut self, reg: Register, v1: Value, v2: Value) -> bool {
        self.run_calc(reg, v1, v2, |a, b| a & b)
    }

    fn run_or(&mut self, reg: Register, v1: Value, v2: Value) -> bool {
        self.run_calc(reg, v1, v2, |a, b| a | b)
    }

    fn run_calc(
        &mut self,
        reg: Register,
        a: Value,
        b: Value,
        calc: impl FnOnce(u16, u16) -> u16,
    ) -> bool {
        let a = self.val(a);
        let b = self.val(b);
        let res = calc(a, b) % MAX_INT;
        self.memory.set_register(reg.into(), res);
        self.cursor += 1;
        false
    }

    fn run_not(&mut self, reg: Register, val: Value) -> bool {
        let val = self.val(val);
        self.memory.set_register(reg.into(), !val % MAX_INT);
        self.cursor += 1;
        false
    }

    fn run_equal(&mut self, reg: Register, v1: Value, v2: Value) -> bool {
        self.run_cond(reg, v1, v2, |a, b| a == b)
    }

    fn run_greater_than(&mut self, reg: Register, v1: Value, v2: Value) -> bool {
        self.run_cond(reg, v1, v2, |a, b| a > b)
    }

    fn run_cond(
        &mut self,
        reg: Register,
        a: Value,
        b: Value,
        comp: impl FnOnce(u16, u16) -> bool,
    ) -> bool {
        let a = self.val(a);
        let b = self.val(b);
        let val = if comp(a, b) { 1 } else { 0 };
        self.memory.set_register(reg.into(), val);
        self.cursor += 1;
        false
    }

    fn run_call(&mut self, val: Value) -> bool {
        let val = self.val(val);
        // 6027 is the test function. Let's dump the program when it's called
        if val == 6027 {
            self.dump();
            self.memory.set_register(0, 6);
            self.cursor += 1;
        } else {
            self.memory.push((self.cursor + 1) as u16);
            self.cursor = val as usize;
        }
        false
    }

    fn run_return(&mut self) -> bool {
        if let Some(addr) = self.memory.pop() {
            self.cursor = addr as usize;
            false
        } else {
            true
        }
    }

    fn run_read_mem(&mut self, reg: Register, addr: Value) -> bool {
        let addr = self.val(addr);
        let val = self.memory.at(addr as usize);
        self.memory.set_register(reg.into(), val);
        self.cursor += 1;
        false
    }

    fn run_write_mem(&mut self, addr: Value, val: Value) -> bool {
        self.memory.set(self.val(addr) as usize, self.val(val));
        self.cursor += 1;
        false
    }

    fn next_value(&mut self) -> Value {
        self.cursor += 1;
        Value::from(self.memory.at(self.cursor))
    }

    // Resolves the value either from a reference or from a literal
    fn val(&self, val: Value) -> u16 {
        match val {
            Value::Register(Register(r)) => self.memory.read_register(r as usize),
            Value::Literal(num) => num,
        }
    }

    fn next_register(&mut self) -> Result<Register> {
        self.cursor += 1;
        let val = Value::from(self.memory.at(self.cursor));
        match val {
            Value::Register(r) => Ok(r),
            Value::Literal(num) => bail!("expected register but got literal {}", num),
        }
    }

    fn dump(&mut self) {
        let initial_cursor = self.cursor;
        self.cursor = 0;
        log::debug!("Dumping program.");
        while self.cursor < MAX_ADDR {
            self.next_operation().ok();
            self.cursor += 1;
        }
        log::debug!("Dumping done.");
        self.cursor = initial_cursor;
    }
}
