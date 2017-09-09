extern crate polygon;
extern crate winit;
extern crate gl_winit;

use gl_winit::CreateContext;
use polygon::gl::GlRender;
use polygon::Renderer;
use winit::*;

fn main() {
    let mut events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();

    // Open a window and create the renderer instance.
    let context = window.create_context().unwrap();
    let mut renderer = GlRender::new(context).unwrap();

    events_loop.run_forever(|event| {
        match event {
            Event::WindowEvent { event: WindowEvent::Closed, .. } => {
                println!("The window was closed ; stopping");
                ControlFlow::Break
            },
            _ => ControlFlow::Continue,
        }
    })
}
