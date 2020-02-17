use core::fmt::{Debug,Formatter,Error};

#[derive(Clone,Copy)]
pub struct Val(pub i32);

impl Debug for Val {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

impl Default for Val {
    fn default() -> Self {
        Self(0)
    }
}

impl Val {
    pub fn from_bool(b: bool) -> Self {
        Self(if b { 1 } else { 0 })
    }
}
