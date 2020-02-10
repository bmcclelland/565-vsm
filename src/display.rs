use glium::{
    Display,
    glutin::{
        window::WindowBuilder,
        ContextBuilder,
        event_loop::EventLoop,
    },
};

pub fn make() -> (Display, EventLoop<()>) {
    let window = WindowBuilder::new()
        .with_resizable(true);
    let context = ContextBuilder::new();
    let event_loop = EventLoop::new();
    let display = Display::new(window, context, &event_loop)
        .unwrap();
    (display, event_loop)
}
