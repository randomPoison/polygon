extern crate polygon;
extern crate winit;

use polygon::gl::GlRender;
use polygon::Renderer;
use winit::*;

fn main() {
    let events_loop = EventsLoop::new();
    let window = Window::new(&events_loop);

    // Open a window and create the renderer instance.
    let mut renderer = GlRender::new(window.context()).unwrap();

    'outer: loop {
        while let Some(message) = window.next_message() {
            if let Message::Close = message {
                break 'outer;
            }
        }

        // Render our empty scene.
        renderer.draw();
    }
}
