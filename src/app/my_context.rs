use glium::Display;
use crate::util::*;
use crate::gl::*;

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

