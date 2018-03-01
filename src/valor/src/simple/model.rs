use std::rc::Rc;
use std::cell::RefCell;
use cgmath::{Matrix4, One, Vector3};
use glium;
use Renderer;
use Handle;
use super::vertex::Vertex;

/// Data structure with vertex data and underlying GPU representation, shared among all instances.
pub struct Model {
    /// The vertex data
    pub vertices: Vec<Vertex>,
    /// A static per-model (not per-instance) transform
    pub transform: Matrix4<f32>,
    /// The vertex buffer sent to the GPU
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    /// Indices on the vertex buffer sent to the GPU
    pub indices: glium::index::NoIndices,
    // pub gpu_data: GpuData,
    /// Flag to indicate the model gpu data needs to be refreshed
    pub is_dirty: bool,
}

impl Model {
    /// Create a new instance of the model
    pub fn new(renderer: &mut Renderer, vertices: &[Vertex]) -> Handle<Self> {
        let vertex_buffer = glium::VertexBuffer::new(&renderer.display, &vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let transform: Matrix4<f32> = Matrix4::one();

        let model = Model {
            vertices: vertices.to_vec(),
            transform,
            vertex_buffer,
            indices,
            is_dirty: false,
        };

        Rc::new(RefCell::new(Box::new(model)))
    }

    /*
    pub fn update_vertices(&mut self, encoder: &mut Encoder) {
        if self.is_dirty {
            // Update params
        }
    }
    */

    // TODO: Do not allow translation on models, but set transform manually
    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.transform = self.transform * Matrix4::from_translation(translation);
    }
}
