use crate::vm::*;
use crate::gl::MeshId;
use crate::util::units::*;

pub struct Ent {
    pub pos: Pos,
    pub scale: Scale,
    pub angle: Radians,
    pub mesh: MeshId,
    pub color: Color,
}

impl Ent {
    pub fn from_inst(inst: Inst, pos: Pos) -> Self {
        let (mesh, scale, color) = inst_gfx(inst);
        let angle = Radians::new(0.0);
        Self { pos, scale, angle, mesh, color, }
    }
    
    pub fn from_mem(mem: Val, pos: Pos) -> Self {
        let (mesh, scale, color) = mem_gfx(mem);
        let angle = Radians::new(0.0);
        Self { pos, scale, angle, mesh, color, }
    }
}

fn inst_gfx(inst: Inst) -> (MeshId, Scale, Color) {
    use Inst::*;
    let scale1 = Scale::new(96.0, 32.0);
    match inst {
        Push(_) => (MeshId::Square, scale1, Color::new(1.0, 1.0, 0.0)),
        Pop     => (MeshId::Square, scale1, Color::new(1.0, 1.0, 1.0)),
        Dup     => (MeshId::Square, scale1, Color::new(0.2, 0.2, 0.2)),
        Dup2    => (MeshId::Square, scale1, Color::new(0.2, 0.2, 0.2)),
        Add     => (MeshId::Square, scale1, Color::new(1.0, 0.0, 0.0)),
        Sub     => (MeshId::Square, scale1, Color::new(0.0, 1.0, 0.0)),
        Mul     => (MeshId::Square, scale1, Color::new(1.0, 0.0, 1.0)),
        Div     => (MeshId::Square, scale1, Color::new(0.5, 1.0, 1.0)),
        CmpLT   => (MeshId::Square, scale1, Color::new(0.0, 1.0, 1.0)),
        CmpLE   => (MeshId::Square, scale1, Color::new(0.0, 1.0, 1.0)),
        CmpEQ   => (MeshId::Square, scale1, Color::new(0.0, 1.0, 1.0)),
        CmpNE   => (MeshId::Square, scale1, Color::new(0.0, 1.0, 1.0)),
        CmpGE   => (MeshId::Square, scale1, Color::new(0.0, 1.0, 1.0)),
        CmpGT   => (MeshId::Square, scale1, Color::new(0.0, 1.0, 1.0)),
        CBR(_)  => (MeshId::Square, scale1, Color::new(0.3, 1.0, 0.0)),
        BR(_)   => (MeshId::Square, scale1, Color::new(1.0, 0.0, 0.3)),
        Print   => (MeshId::Square, scale1, Color::new(1.0, 0.0, 0.5)),
        Peek    => (MeshId::Square, scale1, Color::new(0.0, 1.0, 0.5)),
    }
}

fn mem_gfx(mem: Val) -> (MeshId, Scale, Color) {
    let c: f32 = mem.0 as f32 / 10.0;
    (MeshId::Square, Scale::new(64.0, 32.0), Color::new(c, c, c))
}
