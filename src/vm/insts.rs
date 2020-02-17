use super::val::*;

//#[derive(Clone,Copy,Debug,PartialEq,Eq)]
//pub enum Comparison {
//    LT,
//    LE,
//    EQ,
//    NE,
//    GE,
//    GT,
//}

#[derive(Clone,Copy,Debug)]
pub enum Inst {
    Push(Val),
    Pop,
    Dup,
    Dup2,
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Peek,
    CmpLT,
    CmpLE,
    CmpEQ,
    CmpNE,
    CmpGE,
    CmpGT,
    CBR(usize),
    BR(usize),
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

pub fn lt(a: Val, b: Val) -> Val {
    Val::from_bool(a.0 < b.0)
}

pub fn le(a: Val, b: Val) -> Val {
    Val::from_bool(a.0 <= b.0)
}

pub fn eq(a: Val, b: Val) -> Val {
    Val::from_bool(a.0 == b.0)
}

pub fn ne(a: Val, b: Val) -> Val {
    Val::from_bool(a.0 != b.0)
}

pub fn ge(a: Val, b: Val) -> Val {
    Val::from_bool(a.0 >= b.0)
}

pub fn gt(a: Val, b: Val) -> Val {
    Val::from_bool(a.0 > b.0)
}
