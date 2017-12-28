extern crate gl_winit;
extern crate polygon;
extern crate winit;

use gl_winit::CreateContext;
use polygon::*;
use polygon::anchor::*;
use polygon::camera::*;
use polygon::gl::GlRender;
use polygon::math::*;
use polygon::mesh_instance::*;
use polygon::geometry::mesh::*;
use winit::*;

static VERTEX_POSITIONS: [f32; 12] = [
    -1.0, -1.0, 0.0, 1.0,
     1.0, -1.0, 0.0, 1.0,
     0.0,  1.0, 0.0, 1.0,
];

static INDICES: [u32; 3] = [0, 1, 2];

fn main() {
    // Open a window.
    let mut events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();

    // Create the OpenGL context and the renderer.
    let context = window.create_context().unwrap();
    let mut renderer = GlRender::new(context).unwrap();

    // Build a triangle mesh.
    let mesh = MeshBuilder::new()
        .set_position_data(Point::slice_from_f32_slice(&VERTEX_POSITIONS))
        .set_indices(&INDICES)
        .build()
        .unwrap();

    // Send the mesh to the GPU.
    let gpu_mesh = renderer.register_mesh(&mesh);

    // Create an anchor and register it with the renderer.
    let anchor = Anchor::new();
    let anchor_id = renderer.register_anchor(anchor);

    // Setup the material for the mesh.
    let mut material = renderer.default_material();
    material.set_color("surface_color", Color::rgb(1.0, 0.0, 0.0));

    // Create a mesh instance, attach it to the anchor, and register it.
    let mut mesh_instance = MeshInstance::with_owned_material(gpu_mesh, material);
    mesh_instance.set_anchor(anchor_id);
    let instance_id = renderer.register_mesh_instance(mesh_instance);

    // Create a camera and an anchor for it.
    let mut camera_anchor = Anchor::new();
    camera_anchor.set_position(Point::new(0.0, 0.0, 10.0));
    let camera_anchor_id = renderer.register_anchor(camera_anchor);

    let mut camera = Camera::default();
    camera.set_anchor(camera_anchor_id);
    renderer.register_camera(camera);

    // Set ambient color to pure white so we don't need to worry about lighting.
    renderer.set_ambient_light(Color::rgb(1.0, 1.0, 1.0));

    let mut loop_active = true;
    let mut frames = 0;
    let mut state = State::Visible(instance_id);
    while loop_active {
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event: WindowEvent::Closed, .. } => {
                    loop_active = false;
                }

                _ => {}
            }
        });

        // Rotate the triangle slightly.
        frames += 1;
        if frames == 1000 {
            match state {
                State::Visible(id) => {
                    let instance = renderer.remove_mesh_instance(id).unwrap();
                    state = State::Hidden(instance);
                }

                State::Hidden(mesh_instance) => {
                    let id = renderer.register_mesh_instance(mesh_instance);
                    state = State::Visible(id);
                }
            }

            frames = 0;
        }

        // Render the mesh.
        renderer.draw();
    }
}

enum State {
    Visible(MeshInstanceId),
    Hidden(MeshInstance),
}
