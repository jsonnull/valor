extern crate cgmath;
extern crate glutin;
extern crate valor;

use valor::ValorBuilder;
use valor::camera::Camera;
use valor::scene::{Address, Node, NodeEntry, Scene};
use valor::simple::{Material as SimpleMaterial, Model, Vertex};
use cgmath::Vector3;

const LIGHT_BLUE: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
const RED: [f32; 3] = [1.0, 0.0, 0.0];
const GREEN: [f32; 3] = [0.0, 1.0, 0.0];
const BLUE: [f32; 3] = [0.0, 0.0, 1.0];

pub fn main() {
    let (mut events_loop, mut renderer) = ValorBuilder::new()
        .with_title(&"Basic example")
        .with_dimensions(800, 600)
        .with_clear_color(LIGHT_BLUE)
        .with_vsync(true)
        .finish();

    let camera = Camera::new();

    let mut scene = Scene::new();

    let vertices = &[
        Vertex::new(-0.5, -0.5, 1.0, RED),
        Vertex::new(0.5, -0.5, -1.0, GREEN),
        Vertex::new(0.0, 0.5, 0.0, BLUE),
    ];

    // Create the material we'll draw with
    let material = SimpleMaterial::new(&renderer.display);

    // Create Triangle
    let triangle = Model::new(&mut renderer, vertices);

    let mut triangle_node = Node::new(NodeEntry::Model(triangle));
    triangle_node.translate(Vector3::new(0.0, -0.2, -2.0));
    scene.insert(triangle_node, Address::Root);

    // Create text
    // let text: TextHandle = Text::new("Basic example", [10, 10], WHITE);
    // let text_node = SceneNode::new(SceneNodeEntry::Text(text));
    // scene.insert(text_node, &root_node);

    let mut running = true;
    while running {
        // Update global constant buffer
        let view_proj_matrix: [[f32; 4]; 4] = camera.get_view_proj().into();

        // Draw frame
        renderer.render(|mut target| {
            use valor::Material;

            // Iterate over the entries in the scene graph
            for (node, parent_id) in scene.traverse() {
                match node.entry {
                    NodeEntry::Model(ref model) => {
                        // Update locals with transform
                        // TODO: cache locals on scene graph
                        let u_world: [[f32; 4]; 4] = scene.get_transform(node, parent_id).into();

                        // Ensure usage of the correct material here
                        material.draw(&mut target, model.clone(), view_proj_matrix, u_world);
                    }
                    // SceneNodeEntry::Text(ref text) => {
                        // let text = text.borrow_mut();

                        // Add some text 10 pixels down and right from the top left screen corner.
                        // self.text.add(&text.data, text.position, text.color);

                        // Draw text.
                        // self.text.draw(&mut self.encoder, &self.main_color).unwrap();
                    // },
                    NodeEntry::Empty => {}
                };
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
                | glutin::WindowEvent::Closed => {
                    running = false;
                }
                _ => {}
            },
            _ => {}
        });
    }
}
