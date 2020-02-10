use glium::Display;
use crate::enum_vec::*;
pub use glium::Program;

const VSHADER: &str = r#"
#version 330 core
layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 tc;
out vec3 ourColor;
void main()
{
   gl_Position = vec4(pos, 0.0, 1.0);
   ourColor = vec3(1, 1, 1);
}
"#;

const HSHADER: &str = r#"
#version 330 core
out vec4 FragColor;
in vec3 ourColor;
void main()
{
   FragColor = vec4(ourColor, 1.0f);
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
