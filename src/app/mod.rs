mod ent;

use glium::Display;
use crate::util::*;
use crate::{gl, gl::*};
use crate::vm::*;

pub use ent::Ent;

pub struct App {
    pub meshes:   EnumVec<MeshId, Mesh>,
    pub programs: EnumVec<ProgramId, Program>,
    pub ents: Vec<Ent>,
}

impl App {
    pub fn new(display: &Display) -> Self {
        Self {
            meshes:   gl::init_meshes(display),
            programs: gl::init_programs(display),
            ents:     Vec::new(),
        }
    }

    pub fn push_inst(&mut self, inst: Inst, pos: Pos) {
        self.ents.push(Ent::from_inst(inst, pos));
    }
    
    pub fn push_mem(&mut self, mem: Val, pos: Pos) {
        self.ents.push(Ent::from_mem(mem, pos));
    }
}
