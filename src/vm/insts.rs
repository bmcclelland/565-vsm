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

// TODO just doing this to see it working.
impl Inst {
    pub fn key(self) -> &'static str {
        use Inst::*;
        match self {
            Push(_) => "push",
            Pop => "pop",
            Dup => "dup",
            Dup2 => "dup2",
            Add => "add",
            Sub => "sub",
            Mul => "mul",
            Div => "div",
            Print => "print",
            Peek => "peek",
            CmpLT => "<",
            CmpLE => "<=",
            CmpEQ => "==",
            CmpNE => "!=",
            CmpGE => ">=",
            CmpGT => ">",
            CBR(_) => "cbr",
            BR(_) => "br",
        }
    }

    pub fn arity(self) -> u32 {
        use Inst::*;
        match self {
            Push(_) => 0,
            Pop     => 0,
            Dup     => 1,
            Dup2    => 2,
            Add     => 2,
            Sub     => 2,
            Mul     => 2,
            Div     => 2,
            Print   => 1,
            Peek    => 0,
            CmpLT   => 2,
            CmpLE   => 2,
            CmpEQ   => 2,
            CmpNE   => 2,
            CmpGE   => 2,
            CmpGT   => 2,
            CBR(_)  => 1,
            BR(_)   => 0,
        }
    }
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
