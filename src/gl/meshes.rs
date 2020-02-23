use glium::{ implement_vertex, Display, VertexBuffer };
use crate::util::*;

#[derive(Copy,Clone)]
pub struct Vert {
    pub vert_pos: [f32; 2],
    pub vert_texcoord:  [f32; 2],
}

implement_vertex!(Vert, vert_pos, vert_texcoord);

pub const VERTS: [Vert; 6] = [
    Vert { vert_pos: [-0.5, -0.5], vert_texcoord: [0.0, 0.0] },
    Vert { vert_pos: [ 0.5, -0.5], vert_texcoord: [1.0, 0.0] },
    Vert { vert_pos: [ 0.5,  0.5], vert_texcoord: [1.0, 1.0] },
    Vert { vert_pos: [-0.5, -0.5], vert_texcoord: [0.0, 0.0] },
    Vert { vert_pos: [ 0.5,  0.5], vert_texcoord: [1.0, 1.0] },
    Vert { vert_pos: [-0.5,  0.5], vert_texcoord: [0.0, 1.0] },
];

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
