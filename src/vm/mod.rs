#![allow(dead_code)]

mod insts;
mod val;

use insts::*;
//use val::*;

pub use insts::Inst;
pub use val::Val;

pub struct VM {
    prog:   Vec<Inst>,
    mem:    Vec<Val>,
    ip:     usize,
    output: Vec<String>,
}

impl VM {
    pub fn new(prog: Vec<Inst>) -> Self {
        Self {
            prog,
            mem: Vec::new(),
            ip: 0,
            output: Vec::new(),
        }
    }

    pub fn live(&self) -> bool {
        self.ip < self.prog.len()
    }

    pub fn output(&self) -> &[String] {
        &self.output
    }

    fn pop(&mut self) -> Val {
        self.mem.pop().unwrap()
    }
    
    fn push(&mut self, v: Val) {
        self.mem.push(v);
    }

    pub fn step(&mut self) {
        print!("{:?}\t\t", self.prog[self.ip]);

        use Inst::*;
        match &self.prog[self.ip] {
            Push(val) => {
                self.mem.push(*val);
            }

            Pop => {
                self.mem.pop().unwrap();
            }
           
            Add => self.call2(add),
            Sub => self.call2(sub),
            Mul => self.call2(mul),
            Div => self.call2(div),

            Print => {
                let v = self.pop();
                self.output.push(format!("{:?}", v));
            }
            
            Peek => {
                match self.mem.last() {
                    Some(v) => self.output.push(format!("{:?}", v)),
                    None => self.output.push("(empty)".into()),
                }
            }
        }
            
        println!("{:?}", self.mem);
        self.ip += 1;
    }
    
    fn call1(&mut self, f: fn(Val)->Val) {
        let a = self.pop();
        self.push(f(a));
    }
    
    fn call2(&mut self, f: fn(Val,Val)->Val) {
        let b = self.pop();
        let a = self.pop();
        self.push(f(a, b));
    }
    
    fn call3(&mut self, f: fn(Val,Val,Val)->Val) {
        let c = self.pop();
        let b = self.pop();
        let a = self.pop();
        self.push(f(a,b,c));
    }
}
