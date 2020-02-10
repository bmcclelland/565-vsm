use glium::{
    Surface,
    index::NoIndices,
    index::PrimitiveType,
    uniforms::EmptyUniforms,
    glutin::{
        event_loop::ControlFlow,
        event::*,
    },
};
use std::time::{Duration,Instant};
use vsm::{ MeshId, ProgramId, };

const FRAME_TIME: Duration = Duration::from_nanos(16_666_667);
        
fn main() {
    let (display, event_loop) = vsm::display::make();
    let ctx = vsm::MyContext::new(&display);

    let indices  = NoIndices(PrimitiveType::TrianglesList);
    let uniforms = EmptyUniforms;
    let draw_params = Default::default();

    event_loop.run(move |event, _, control_flow| {
        wait_for_frame(control_flow);

        use VirtualKeyCode::*;
        match get_key(&event) {
            Some(Q) => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    
        let mesh    = &ctx.meshes[MeshId::Square];
        let program = &ctx.programs[ProgramId::Std];

        let mut target = display.draw();
        target.clear_color(0.1, 0.1, 0.4, 1.0);
        target.draw(mesh, &indices, program, &uniforms, &draw_params)
            .unwrap();
        target.finish()
            .unwrap();
    });
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
    
