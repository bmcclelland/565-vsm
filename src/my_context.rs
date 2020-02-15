use glium::Display;
use crate::util::*;
use super::meshes::*;
use super::programs::*;

use nalgebra_glm as glm;

pub type Pos = glm::Vec2;
pub type Scale = glm::Vec2;

#[derive(Copy,Clone)]
pub struct Radians(pub f32);

pub struct Ent {
    pub pos: Pos,
    pub scale: Scale,
    pub angle: Radians,
    pub mesh: MeshId,
}

pub struct MyContext {
    pub meshes:   EnumVec<MeshId, Mesh>,
    pub programs: EnumVec<ProgramId, Program>,
    pub ents: Vec<Ent>,
}

impl MyContext {
    pub fn new(display: &Display) -> Self {
        Self {
            meshes:   init_meshes(display),
            programs: init_programs(display),
            ents:     Vec::new(),
        }
    }
}
