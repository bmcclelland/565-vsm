mod ent;

use glium::Display;
use crate::util::*;
use crate::{gl, gl::*};
use crate::vm::*;
use crate::consts::*;
use nalgebra_glm::Vec2;

pub use ent::Ent;

pub struct App {
    pub meshes:   EnumVec<MeshId, Mesh>,
    pub programs: EnumVec<ProgramId, Program>,
    pub ents: Vec<Ent>,
    pub vm: VM,
}

impl App {
    pub fn new(display: &Display) -> Self {
        Self {
            meshes:   gl::init_meshes(display),
            programs: gl::init_programs(display),
            ents:     Vec::new(),
            vm:       VM::new(),
        }
    }

    pub fn render_prog(&mut self) {
        self.ents.clear();

        // Current inst is at (0,0). Will displace later.
        let mut pos = Vec2::new(
            self.vm.pc() as f32 * -INST_WIDTH_F,
            0.0,
        );

        for &inst in self.vm.insts() {
            self.ents.append(&mut inst_base(inst, pos));
            self.ents.append(&mut inst_text(inst, pos));
            pos.x += INST_WIDTH_F;
        }

        // Recenter on current inst.
        pos.x = 0.0;
        pos.y -= INST_HEIGHT_F;
        for &mem in self.vm.mem().rev() {
            self.ents.append(&mut mem_base(mem, pos));
            self.ents.append(&mut mem_text(mem, pos));
            pos.y -= INST_HEIGHT_F;
        }
    }
}

fn inst_base(inst: Inst, pos: Pos) -> Vec<Ent> {
    let mesh  = MeshId::Square;
    let scale = Scale::new(INST_WIDTH_F, INST_HEIGHT_F);
    let color = match inst.arity() {
        0 => Color::new(0.5, 0.0, 1.0),
        1 => Color::new(1.0, 1.0, 0.0),
        2 => Color::new(1.0, 0.0, 0.0),
        _ => Color::new(0.0, 1.0, 1.0),
    };
    let angle = Radians::new(0.0);
    let tex = "inst";
    vec![Ent { pos, scale, angle, mesh, color, tex, natural: false }]
}

fn inst_text(inst: Inst, pos: Pos) -> Vec<Ent> {
    let mesh  = MeshId::Square;
    let scale = Scale::new(0.666, 0.666); // TODO
    let color = Color::new(1.0, 1.0, 1.0);
    let angle = Radians::new(0.0);
    let tex = inst.key();
    vec![Ent { pos, scale, angle, mesh, color, tex, natural: true }]
}

fn mem_base(_mem: Val, pos: Pos) -> Vec<Ent> {
    let mesh  = MeshId::Square;
    let scale = Scale::new(INST_WIDTH_F, INST_HEIGHT_F);
    let color = Color::new(1.0, 1.0, 1.0);
    let angle = Radians::new(0.0);
    let tex   = "val";
    vec![Ent { pos, scale, angle, mesh, color, tex, natural: false }]
}

fn mem_text(mem: Val, mem_pos: Pos) -> Vec<Ent> {
    let mesh  = MeshId::Square;
    let scale = Scale::new(DIGIT_WIDTH_F, DIGIT_HEIGHT_F);
    let color = Color::new(1.0, 1.0, 1.0);
    let angle = Radians::new(0.0);

    let digits = break_digits(mem.0);
    let digits_x0 = if digits.len() % 2 == 0 {
        assert!(digits.len() >= 2);
        let offn = digits.len() as u32 / 2 - 1;
        mem_pos.x - offn as f32 * DIGIT_WIDTH_F - DIGIT_WIDTH_F / 2.0
    }
    else {
        let offn = digits.len() as u32 / 2;
        mem_pos.x - offn as f32 * DIGIT_WIDTH_F
    };

    let mut ents = Vec::new();
    let mut pos = Vec2::new(digits_x0, mem_pos.y);

    for d in digits.into_iter() {
        let tex = digit_tex(d);
        ents.push(Ent { pos, scale, angle, mesh, color, tex, natural: false });
        pos.x += DIGIT_WIDTH_F;
    }

    ents
}

fn digit_tex(d: i32) -> &'static str {
    assert!(d >= 0 && d < 10);
    match d {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        _ => "?",
    }
}

fn break_digits(mut x: i32) -> Vec<i32> {
    let mut out = Vec::new();
    loop {
        out.push(x % 10);

        if x < 10 {
            out.reverse();
            assert!(!out.is_empty());
            return out;
        }

        x /= 10;
    }
}

#[cfg(test)]
#[test]
fn test_break_digits() {
    assert_eq!(break_digits(0),   vec![0]);
    assert_eq!(break_digits(3),   vec![3]);
    assert_eq!(break_digits(10),  vec![1,0]);
    assert_eq!(break_digits(11),  vec![1,1]);
    assert_eq!(break_digits(33),  vec![3,3]);
    assert_eq!(break_digits(100), vec![1,0,0]);
    assert_eq!(break_digits(199), vec![1,9,9]);
}
