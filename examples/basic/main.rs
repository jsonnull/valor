extern crate glutin;
extern crate valor;
extern crate cgmath;

use valor::{ValorBuilder, Camera, Scene, SceneNode, SceneNodeEntry, Model, ModelHandle, Text,
            TextHandle, Vertex};
use glutin::GlContext;
use cgmath::Vector3;

const LIGHT_BLUE: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED: [f32; 3] = [1.0, 0.0, 0.0];
const GREEN: [f32; 3] = [0.0, 1.0, 0.0];
const BLUE: [f32; 3] = [0.0, 0.0, 1.0];

pub fn main() {
    let (window, mut events_loop, mut renderer) = ValorBuilder::new()
        .with_title(&"Basic example")
        .with_dimensions(800, 600)
        .with_clear_color(LIGHT_BLUE)
        .with_vsync(true)
        .finish();

    let camera = Camera::new();

    let mut scene = Scene::new();
    let root_node = scene.get_root_id();

    let vertices: Vec<Vertex> = vec![
        Vertex::new(-0.5, -0.5, 1.0, RED),
        Vertex::new(0.5, -0.5, -1.0, GREEN),
        Vertex::new(0.0, 0.5, 0.0, BLUE),
    ];

    // Create Triangle and Text
    let triangle: ModelHandle = Model::new(&mut renderer, &vertices);
    let text: TextHandle = Text::new("Basic example", [10, 10], WHITE);

    let mut triangle_node = SceneNode::new(SceneNodeEntry::Model(triangle));
    triangle_node.translate(Vector3::new(0.0, -0.2, -2.0));

    let text_node = SceneNode::new(SceneNodeEntry::Text(text));

    scene.insert(triangle_node, &root_node);
    scene.insert(text_node, &root_node);

    let mut running = true;
    while running {
        // Draw frame
        renderer.render(&mut scene, &camera);
        window.swap_buffers().unwrap();

        // Handle events
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Resized(width, height) => {
                        renderer.set_dimensions(width, height);
                        renderer.update_views(&window);
                    }
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape), ..
                        },
                        ..
                    } |
                    glutin::WindowEvent::Closed => {
                        running = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        });
    }
}
