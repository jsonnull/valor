use valor::cgmath::Vector3;
use valor::simple::{Material as SimpleMaterial, Model, Vertex};
use valor::{glutin, Handle, ValorBuilder};
use valor_camera::Camera;
use valor_scene::Scene;

const LIGHT_BLUE: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
const RED: [f32; 3] = [1.0, 0.0, 0.0];
const GREEN: [f32; 3] = [0.0, 1.0, 0.0];
const BLUE: [f32; 3] = [0.0, 0.0, 1.0];

enum SceneEntry {
    Model(Handle<Model>),
    Empty,
}

pub fn main() {
    let (mut events_loop, mut renderer) = ValorBuilder::new()
        .with_title(&"Basic example")
        .with_dimensions(800.0, 600.0)
        .with_clear_color(LIGHT_BLUE)
        .with_vsync(true)
        .finish();

    let camera = Camera::new();

    // Create a scene with string ids
    let mut scene = Scene::new(SceneEntry::Empty);
    let root = scene.get_root();

    let vertices = &[
        Vertex::new(-0.5, -0.5, 1.0, RED),
        Vertex::new(0.5, -0.5, -1.0, GREEN),
        Vertex::new(0.0, 0.5, 0.0, BLUE),
    ];

    // Create the material we'll draw with
    let material = SimpleMaterial::new(&renderer.display);

    // Create Triangle
    let triangle = Model::new(&mut renderer, vertices);

    let triangle_index = scene.create_node(SceneEntry::Model(triangle));
    scene.translate(triangle_index, Vector3::new(0.0, -0.2, -2.0));
    scene.add_child(root, triangle_index);

    // scene.find(ROOT).unwrap().add_child(triangle_node);

    let mut running = true;
    while running {
        // Update global constant buffer
        let view_proj_matrix: [[f32; 4]; 4] = camera.get_view_proj().into();

        // Draw frame
        renderer.render(|mut target| {
            use valor::Material;

            // Iterate over the entries in the scene graph
            for (_id, entry, transform) in scene.traverse() {
                match entry {
                    SceneEntry::Model(ref model) => {
                        // Update locals with transform
                        // TODO: cache locals on scene graph
                        let u_world: [[f32; 4]; 4] = transform.into();

                        // Ensure usage of the correct material here
                        material.draw(&mut target, model.clone(), view_proj_matrix, u_world);
                    }
                    SceneEntry::Empty => {}
                }
            }
        });

        // Handle events
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::KeyboardInput {
                    input:
                        glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                }
                | glutin::WindowEvent::CloseRequested => {
                    running = false;
                }
                _ => {}
            },
            _ => {}
        });
    }
}
