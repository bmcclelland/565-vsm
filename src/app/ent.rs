use crate::gl::MeshId;
use crate::util::units::*;

pub struct Ent {
    pub pos: Pos,
    pub scale: Scale,
    pub angle: Radians,
    pub mesh: MeshId,
    pub color: Color,
    pub tex: &'static str,
    pub natural: bool,
}
