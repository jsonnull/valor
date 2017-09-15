use std::rc::Rc;
use std::cell::RefCell;
use gfx::traits::FactoryExt;
use cgmath::{Matrix4, Vector3, One};
use Renderer;
use Vertex;
use defines::GpuData;

/// Data structure with vertex data and underlying GPU representation, shared among all instances.
pub struct Model {
    /// The vertex data
    pub vertices: Vec<Vertex>,
    /// A static per-model (not per-instance) transform
    pub transform: Matrix4<f32>,
    /// The params which are sent through the pipeline to the shader program
    pub gpu_data: GpuData,
    /// Flag to indicate the model gpu data needs to be refreshed
    pub is_dirty: bool,
}

pub type ModelHandle = Rc<RefCell<Model>>;

impl Model {
    /// Create a new instance of the model
    pub fn new(mut renderer: &mut Renderer, vertices: &[Vertex]) -> ModelHandle {
        let (vertex_buffer, slice) = renderer.factory.create_vertex_buffer_with_slice(
            vertices,
            (),
        );

        let locals = renderer.factory.create_constant_buffer(1);

        let gpu_data = GpuData {
            slice,
            vertices: vertex_buffer,
            locals,
        };

        let transform: Matrix4<f32> = Matrix4::one();

        Rc::new(RefCell::new(Model {
            vertices: vertices.to_vec(),
            transform,
            gpu_data,
            is_dirty: false,
        }))
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
