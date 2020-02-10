use glium::{
    Surface,
    uniform,
    index::NoIndices,
    index::PrimitiveType,
    glutin::{
        event_loop::ControlFlow,
        event::*,
    },
};
use std::time::{Duration,Instant};
use vsm::{ MeshId, ProgramId, };
use nalgebra_glm as glm;

const FRAME_TIME: Duration = Duration::from_nanos(16_666_667);

struct Radians(pub f32);
        
fn main() {
    let (display, event_loop) = vsm::display::make();
    let ctx = vsm::MyContext::new(&display);

    let indices  = NoIndices(PrimitiveType::TrianglesList);
    let draw_params = Default::default();

    let mut rad = 0.0;

    event_loop.run(move |event, _, control_flow| {
        wait_for_frame(control_flow);

        use VirtualKeyCode::*;
        match get_key(&event) {
            Some(Q) => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    
        let mesh    = &ctx.meshes[MeshId::Square];
        let program = &ctx.programs[ProgramId::Std];
        let mvp = make_mvp(
            glm::vec2(-0.5, 0.5),
            glm::vec2(0.5, 0.5),
            Radians(rad),
        );
        rad += 0.001;
        let uniforms = uniform! {
            u_mvp: *mvp.as_ref()
        };

        let mut target = display.draw();
        target.clear_color(0.1, 0.1, 0.4, 1.0);
        target.draw(mesh, &indices, program, &uniforms, &draw_params)
            .unwrap();
        target.finish()
            .unwrap();
    });
}

fn make_mvp(
    mpos: glm::Vec2,
    mscale: glm::Vec2,
    mangle: Radians,
    ) -> glm::Mat4
{
    let t = glm::translation(&glm::vec3(mpos.x, mpos.y, 0.0));
    let s = glm::scaling(&glm::vec3(mscale.x, mscale.y, 1.0));
    let r = glm::rotation(mangle.0, &glm::vec3(0.0, 0.0, 1.0));
    t * r * s * glm::Mat4::identity()
}

fn wait_for_frame(cf: &mut ControlFlow) {
    let next_frame_time = Instant::now() + FRAME_TIME;
    *cf = ControlFlow::WaitUntil(next_frame_time);
}
   
fn get_key<T>(event: &Event<T>) -> Option<VirtualKeyCode> {
    match event {
        Event::WindowEvent { 
            event: WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    virtual_keycode,
                    ..
                },
                ..
            },
            ..
        } => { return *virtual_keycode; }
        _ => { return None }
    }
}
    
