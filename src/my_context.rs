use glium::Display;
use crate::enum_vec::*;
use super::meshes::*;
use super::programs::*;

pub struct MyContext {
    pub meshes:   EnumVec<MeshId, Mesh>,
    pub programs: EnumVec<ProgramId, Program>,
}

impl MyContext {
    pub fn new(display: &Display) -> Self {
        Self {
            meshes:   init_meshes(display),
            programs: init_programs(display),
        }
    }
}
