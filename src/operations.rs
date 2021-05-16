use crate::values::{Register, Value};

#[derive(Debug)]
pub enum Operation {
    Halt,
    Set(Register, Value),
    Push(Value),
    Pop(Register),
    Equal(Register, Value, Value),
    GreaterThan(Register, Value, Value),
    Jump(Value),
    JumpTrue(Value, Value),
    JumpFalse(Value, Value),
    Add(Register, Value, Value),
    Multiply(Register, Value, Value),
    Modulo(Register, Value, Value),
    And(Register, Value, Value),
    Or(Register, Value, Value),
    Not(Register, Value),
    ReadMem(Register, Value),
    WriteMem(Value, Value),
    Call(Value),
    Return,
    Out(Value),
    Input(Register),
    Noop,
}
