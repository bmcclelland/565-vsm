#![allow(dead_code)]

mod insts;
mod val;

use insts::*;
//use val::*;

pub use insts::Inst;
pub use val::Val;
pub type Prog = Vec<Inst>;

pub struct VM {
    prog:    Prog,
    mem:     Vec<Val>,
    pc:      usize,
    next_pc: usize,
    output:  Vec<String>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            prog:    Vec::new(),
            mem:     Vec::new(),
            pc:      0,
            next_pc: 0,
            output:  Vec::new(),
        }
    }

    pub fn load(&mut self, prog: Prog) {
        self.prog = prog;
        self.mem.clear();
        self.pc = 0;
        self.next_pc = 0;
        self.output.clear();
    }

    pub fn insts(&self) -> std::slice::Iter<Inst> {
        self.prog.iter()
    }

    pub fn mem(&self) -> std::slice::Iter<Val> {
        self.mem.iter()
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn live(&self) -> bool {
        self.pc < self.prog.len()
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

    pub fn advance(&mut self) {
        self.pc = self.next_pc;
    }

    pub fn exec(&mut self) {
        print!("{:?}\t\t", self.prog[self.pc]);

        use Inst::*;
        self.next_pc = self.pc + 1;
        match &self.prog[self.pc] {
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
                    self.next_pc = i;
                }
            }

            BR(i) => {
                self.next_pc = *i;
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
