#[derive(Debug)]
pub struct Register(pub u8);

impl From<Register> for usize {
    fn from(reg: Register) -> usize {
        let Register(val) = reg;
        val as usize
    }
}

#[derive(Debug)]
pub enum Value {
    Literal(u16),
    Register(Register),
}

impl From<u16> for Value {
    fn from(raw: u16) -> Self {
        match raw {
            r if r <= 32767 => Value::Literal(raw),
            r if r <= 32775 => Value::Register(Register((r - 32768) as u8)),
            _ => panic!("Invalid value {}", raw),
        }
    }
}

impl From<Value> for u16 {
    fn from(val: Value) -> u16 {
        match val {
            Value::Literal(v) => v,
            Value::Register(Register(v)) => v as u16,
        }
    }
}
