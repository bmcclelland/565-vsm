use glium::{ implement_vertex, Display, VertexBuffer };
use crate::enum_vec::*;

#[derive(Copy,Clone)]
pub struct Vert {
    pub pos: [f32; 2],
    pub tc:  [f32; 2],
}

implement_vertex!(Vert, pos, tc);

pub const VERTS: [Vert; 3] = [
    Vert { pos: [0.0, 0.0], tc: [0.0, 0.0] },
    Vert { pos: [1.0, 0.0], tc: [0.0, 0.0] },
    Vert { pos: [0.0, 1.0], tc: [0.0, 0.0] },
];

pub fn square(display: &Display) -> VertexBuffer<Vert> {
    VertexBuffer::new(display, &VERTS).unwrap()
}

smart_enum! { MeshId: u8 =
    Square,
}

pub type Mesh = VertexBuffer<Vert>;
   
pub fn init_meshes(display: &Display) -> EnumVec<MeshId, Mesh> {
    macro_rules! mesh(
        ($val:ident) => {
            Mesh::new(display, &$val).unwrap();
        }
    );

    enum_vec!(MeshId -> Mesh {
        MeshId::Square => mesh!(VERTS),
    })
}
