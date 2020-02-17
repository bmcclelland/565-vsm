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

    pub fn insts(&self) -> impl Iterator<Item = &Inst> {
        self.prog.iter()
    }
    
    pub fn mem(&self) -> impl Iterator<Item = &Val> {
        self.mem.iter()
    }

    pub fn ip(&self) -> usize {
        self.ip
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
        let mut next_ip = self.ip + 1;
        match &self.prog[self.ip] {
            Push(val) => {
                self.mem.push(*val);
            }

            Pop => {
                self.mem.pop().unwrap();
            }

            Dup => {
                self.mem.push(*self.mem.last().unwrap());
            }
            
            Dup2 => {
                let n = self.mem.len();
                assert!(n >= 2);
                self.mem.push(self.mem[n-2]);
                self.mem.push(self.mem[n-1]);
            }
           
            Add => self.call2(add),
            Sub => self.call2(sub),
            Mul => self.call2(mul),
            Div => self.call2(div),
            CmpLT => self.call2(lt),
            CmpLE => self.call2(le),
            CmpEQ => self.call2(eq),
            CmpNE => self.call2(ne),
            CmpGE => self.call2(ge),
            CmpGT => self.call2(gt),
            
            CBR(i) => {
                let i = *i;
                if self.pop().0 != 0 {
                    next_ip = i;
                }
            }
            
            BR(i) => {
                next_ip = *i;
            }
            
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
        self.ip = next_ip;
    }
    
    fn call1(&mut self, f: fn(Val)->Val) {
        let a = self.pop();
        self.push(f(a));
        self.ip += 1;
    }
    
    fn call2(&mut self, f: fn(Val,Val)->Val) {
        let b = self.pop();
        let a = self.pop();
        self.push(f(a, b));
        self.ip += 1;
    }
    
    fn call3(&mut self, f: fn(Val,Val,Val)->Val) {
        let c = self.pop();
        let b = self.pop();
        let a = self.pop();
        self.push(f(a,b,c));
        self.ip += 1;
    }
}
