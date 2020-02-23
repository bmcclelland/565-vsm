use glium::Display;
use crate::util::*;
pub use glium::Program;

const VSHADER: &str = r#"
    #version 330 core

    uniform mat4 u_mvp;
    uniform vec3 u_color;
    uniform vec2 u_tpos;
    uniform vec2 u_tscale;
    
    layout (location = 0) in vec2 vert_pos;
    layout (location = 1) in vec2 vert_texcoord;
   
    smooth out vec2 frag_texcoord;
    out vec3 frag_color;

    void main()
    {
       gl_Position = u_mvp * vec4(vert_pos, 0.0, 1.0);
       frag_color = u_color;
       frag_texcoord = u_tpos + vert_texcoord * u_tscale;        
    }
"#;

const HSHADER: &str = r#"
    #version 330 core

    uniform sampler2D u_sampler;

    in vec3 frag_color;
    smooth in vec2 frag_texcoord;
    
    out vec4 out_color;

    void main()
    {
        vec2 flipped_texcoord = vec2(
            frag_texcoord.x, 
            1 - frag_texcoord.y
        );

        vec4 tex_color = vec4(frag_color, 1.0) * texture2D(u_sampler, flipped_texcoord);
        out_color = tex_color;
    }
"#;

smart_enum! { ProgramId: u8 =
    Std,
}

pub fn init_programs(display: &Display) -> EnumVec<ProgramId, Program> {
    macro_rules! program(
        ($vert:ident, $frag:ident) => {
            Program::from_source(display, $vert, $frag, None).unwrap();
        }
    );

    enum_vec!(ProgramId -> Program {
        ProgramId::Std => program!(VSHADER, HSHADER),
    })
}
