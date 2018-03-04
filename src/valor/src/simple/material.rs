use glium;
use Material;
use Handle;

pub use super::Model;

/// Implementor of `Material`, used to draw models in the `simple` module
pub struct SimpleMaterial {
    program: glium::program::Program,
}

impl SimpleMaterial {
    /// Create a new instance
    pub fn new(display: &glium::Display) -> Self {
        let program = glium::Program::from_source(
            display,
            include_str!("shaders/triangle_150_vs.glsl"),
            include_str!("shaders/triangle_150_fs.glsl"),
            None,
        ).unwrap();

        SimpleMaterial { program }
    }
}

impl Material<Model> for SimpleMaterial {
    fn draw(
        &self,
        target: &mut glium::Frame,
        model: Handle<Model>,
        u_view_proj: [[f32; 4]; 4],
        u_world: [[f32; 4]; 4],
    ) {
        use glium::Surface;

        let md = model.borrow_mut();

        let uniforms = uniform! {
            u_ViewProj: u_view_proj,
            u_World: u_world
        };

        target
            .draw(
                &md.vertex_buffer,
                &md.indices,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
