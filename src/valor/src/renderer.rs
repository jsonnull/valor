use glutin;
use gfx_window_glutin;
use gfx_device_gl;
use gfx;
use gfx_text;
use cgmath::{Matrix4, One};
use id_tree::NodeId;
use types::*;
use RenderGroup;
use Scene;
use SceneNode;
use SceneNodeEntry;
use Camera;
use defines::{pipe, Locals, Globals};

/// Interface responsible for window creation and scene traversal for drawing.
pub struct Renderer {
    pub factory: Factory,
    device: Device,
    encoder: Encoder,

    /// Output framebuffer
    main_color: OutputColor,
    /// Output depth buffer
    main_depth: OutputDepth,

    /// Color for OpenGL clear command
    clear_color: [f32; 4],

    /// Width of the frame
    pub width: u32,
    /// Height of the frame
    pub height: u32,

    /// Global data for a given frame
    globals: gfx::handle::Buffer<Resources, Globals>,

    /// Text renderer
    text: gfx_text::Renderer<Resources, gfx_device_gl::Factory>,

    // All possible pipelines:
    /// The pipeline for drawing with the `Simple` group
    pso_simple: PipelineState<pipe::Meta>,
}

impl Renderer {
    /// Create a new rendering instance. You should probably use `ValorBuilder` instead of this method.
    pub(crate) fn new(
        factory: Factory,
        device: Device,
        encoder: Encoder,
        main_color: OutputColor,
        main_depth: OutputDepth,
        clear_color: [f32; 4],
        width: u32,
        height: u32,
        globals: gfx::handle::Buffer<Resources, Globals>,
        text: gfx_text::Renderer<Resources, gfx_device_gl::Factory>,
        pso_simple: PipelineState<pipe::Meta>,
    ) -> Self {
        Renderer {
            device,
            factory,
            main_color,
            main_depth,
            clear_color,
            encoder,
            text,
            pso_simple,
            globals,
            width,
            height,
        }
    }

    /// Update the output buffers based on the window width and height.
    pub fn update_views(&mut self, window: &glutin::GlWindow) {
        gfx_window_glutin::update_views(window, &mut self.main_color, &mut self.main_depth);
    }

    /// Setter for the frame dimensions.
    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Traverse the scene graph and render all the nodes from the camera's perspective.
    pub fn render(&mut self, scene: &mut Scene, camera: &Camera) {
        use gfx::Device;

        let globals = Globals { mx_vp: camera.get_view_proj().into() };
        self.encoder.update_constant_buffer(&self.globals, &globals);

        // Clear background color
        self.encoder.clear(&self.main_color, self.clear_color);

        // Iterate over the entries in the scene graph
        for node_container in scene.traverse() {
            // Get the node which has the model and transform data
            let node: &SceneNode = node_container.data();
            let parent_id = node_container.parent();

            match node.entry {
                SceneNodeEntry::Model(ref model) => {
                    // Fetch correct PSO to draw with
                    let pso = match node.group {
                        RenderGroup::Simple => &self.pso_simple,
                        _ => &self.pso_simple,
                    };

                    // Update locals with transform
                    let transform = self.get_transform(scene, node, parent_id);
                    let locals = Locals { mx_world: transform.into() };
                    // TODO: cache locals on scene graph

                    // Flush updated locals into gpu buffers
                    let model = model.borrow_mut();
                    self.encoder.update_constant_buffer(
                        &model.gpu_data.locals,
                        &locals,
                    );

                    let params = pipe::Data {
                        vbuf: model.gpu_data.vertices.clone(),
                        cb_locals: model.gpu_data.locals.clone(),
                        cb_globals: self.globals.clone(),
                        out: self.main_color.clone(),
                    };

                    self.encoder.draw(&model.gpu_data.slice, pso, &params);
                },
                SceneNodeEntry::Text(ref text) => {
                    let text = text.borrow_mut();

                    // Add some text 10 pixels down and right from the top left screen corner.
                    self.text.add(&text.data, text.position, text.color);

                    // Draw text.
                    self.text.draw(&mut self.encoder, &self.main_color).unwrap();
                },
                SceneNodeEntry::Empty => {}
            };
        }

        self.encoder.flush(&mut self.device);
        self.device.cleanup();
    }

    /// Get ancestor transforms for a given node 
    fn get_transform (&self, scene: &Scene, node: &SceneNode, parent_id: Option<&NodeId>) -> Matrix4<f32> {
        let identity: Matrix4<f32> = Matrix4::one();

        // Pool transforms for ancestors of the parent
        let mut transforms: Vec<Matrix4<f32>> = vec![];
        if let Some(id) = parent_id {
            // Get ancestor transforms
            transforms = scene
                .graph
                .ancestors(id)
                .unwrap()
                .map(|n| n.data().transform)
                .collect();

            transforms.reverse();

            // Get parent transform
            let parent = scene.graph.get(id).unwrap();
            transforms.push(parent.data().transform);
        };

        transforms.push(node.transform);

        let transform = transforms.iter().fold(identity, |acc, t| acc * t);

        transform
    }
}
