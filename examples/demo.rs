use gfx::{glue, QuadContext};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let ev = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(500, 100))
        .build(&ev)
        .unwrap();

    let gl_context = unsafe { glue::GlContext::create(glue::GlConfig::default(), &window) }.unwrap();
    gl_context.make_current();
    gl_context.set_swap_interval(true);

    let gl = gl_context.glow();
    let mut graphics = QuadContext::new(gl);

    ev.run(move |event, _, flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => flow.set_exit(),
            WindowEvent::Resized(new) => {
                graphics.resize(new.width as i32, new.height as i32);
            }
            _ => {}
        },
        Event::MainEventsCleared => {
            graphics.begin_default_pass(Default::default());
            graphics.commit_frame();
            gl_context.swap_buffers();
        }
        _ => {}
    })
}
