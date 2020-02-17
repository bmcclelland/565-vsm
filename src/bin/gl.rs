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
use vsm::{ 
    gl::*,
    util::*,
    app::*,
    vm::*,
};
use nalgebra_glm as glm;
use glm::{
    Vec2,Mat4,
};

const FRAME_TIME: Duration = Duration::from_nanos(16_666_667);
const Z_NEAR: f32 = 1.0;
const Z_FAR: f32 = 1000.0;

fn main() {
    let (display, event_loop) = vsm::gl::make_display();
    let mut app = App::new(&display);
    let mut vm  = make_vm();
    let mut vm_tick = 0;

    let indices  = NoIndices(PrimitiveType::TrianglesList);
    let draw_params = Default::default();

    event_loop.run(move |event, _, control_flow| {
        wait_for_frame(control_flow);

        use VirtualKeyCode::*;
        match get_key(&event) {
            Some(Q) => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    
        let mut target = display.draw();
        target.clear_color(0.1, 0.1, 0.4, 1.0);

        app.ents.clear();
        let mut pos = Vec2::new(0.0, 0.0);
        for inst in vm.insts().skip(vm.ip()) {
            app.push_inst(*inst, pos);
            pos.y += 32.0;
        }

        pos = Vec2::new(-100.0, 0.0);
        for mem in vm.mem() {
            app.push_mem(*mem, pos);
            pos.y += 32.0;
        }
        
        for ent in app.ents.iter() {
            let mesh = &app.meshes[ent.mesh];
            let program = &app.programs[ProgramId::Std];
            let mvp = make_mvp(ent.pos, ent.scale, ent.angle);
            let uniforms = uniform! {
                u_mvp: *mvp.as_ref(),
                u_color: *ent.color.as_ref(),
            };

            target.draw(mesh, &indices, program, &uniforms, &draw_params)
                .unwrap();
        }
        
        target.finish().unwrap();

        vm_tick += 1;
        if vm_tick >= 300 {
            vm_tick = 0;
            if vm.live() {
                vm.step();
            }
        }
    });
}
    
use std::f32::consts::PI;
const VIEW_RES_X: f32 = 800.0;
const VIEW_RES_Y: f32 = 600.0;
const DRAW_RES_X: f32 = 400.0;
const DRAW_RES_Y: f32 = 300.0;
const ASPECT: f32 = DRAW_RES_X / DRAW_RES_Y;
const FOV: f32 = PI / 4.0;
const UP_ANGLE: f32 = PI / 2.0;

fn make_mvp(
    mpos: glm::Vec2,
    mscale: glm::Vec2,
    mangle: Radians,
    ) -> glm::Mat4
{
    let viewport = Vec2::new(VIEW_RES_X, VIEW_RES_Y);
    let cam = Vec2::new(0.0, 0.0);
    let dim = Vec2::new(viewport.x / 2.0, viewport.y / 2.0);
    let view = parallax_ortho_cam(&cam, &dim, 0.0);
    let proj = glm::perspective(FOV, ASPECT, Z_NEAR, Z_FAR);
    
    let t = glm::translation(&glm::vec3(mpos.x, mpos.y, 0.0));
    let s = glm::scaling(&glm::vec3(mscale.x, mscale.y, 1.0));
    let r = glm::rotation(mangle.float, &glm::vec3(0.0, 0.0, 1.0));
    let model = t * r * s * glm::Mat4::identity();

    proj * view * model
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
   
fn parallax_ortho(mut x: f32, mut y: f32, w: f32, h: f32, height: f32) -> Mat4 {
    if height != 0.0 {
        const CAM_H: f32 = 1.0;
        const CAM_A: f32 = std::f32::consts::PI / 6.0;
        let s = CAM_A.sin();

        if s == 0.0 {
            x = 0.0;
            y = 0.0;
        }
        else {
            let ratio = (s * CAM_H) 
                       / (s * (CAM_H - height));
            x *= ratio;
            y *= ratio;
        }
    }

    glm::ortho(x - w, x + w, y - h, y + h, Z_NEAR, Z_FAR)
}

fn parallax_ortho_cam(cam: &Vec2, dim: &Vec2, height: f32) -> Mat4 {
    parallax_ortho(cam.x, cam.y, dim.x, dim.y, height)
}

fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

fn make_vm() -> VM {
    use vsm::vm::*;
    use Inst::*;

    let prog = vec![
        Push(Val(1)),
        Peek,
        Dup,
        Push(Val(5)),
        CmpGE,
        CBR(9),
        Push(Val(1)),
        Add,
        BR(1),
        Print,
    ];

    VM::new(prog)
}
