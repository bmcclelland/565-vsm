use super::val::*;

#[derive(Clone,Copy,Debug)]
pub enum Inst {
    Push(Val),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Peek,
}

pub fn add(a: Val, b: Val) -> Val {
    Val(a.0 + b.0)
}

pub fn sub(a: Val, b: Val) -> Val {
    Val(a.0 - b.0)
}

pub fn mul(a: Val, b: Val) -> Val {
    Val(a.0 * b.0)
}

pub fn div(a: Val, b: Val) -> Val {
    Val(a.0 / b.0)
}
