use nalgebra_glm as glm;

pub type Pos = glm::Vec2;
pub type Scale = glm::Vec2;
pub type Color = glm::Vec3;

#[derive(Copy,Clone)]
pub struct Radians {
    pub float: f32,
}

impl Default for Radians {
    fn default() -> Self {
        Self {
            float: 0.0,
        }
    }
}

impl Radians {
    pub fn new(float: f32) -> Self {
        Self {
            float
        }
    }
}
