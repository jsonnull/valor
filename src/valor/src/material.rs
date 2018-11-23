use crate::Handle;
use glium;

/// A material performs the drawing operation for a model.
///
/// Different implementors of this trait may handle drawing differently.
pub trait Material<T> {
    /// Draw a model.
    ///
    /// Currently this trait assumes specific knowledge of data required to
    /// perform the draw, but that will go away in future versions.
    fn draw(
        &self,
        target: &mut glium::Frame,
        model: Handle<T>,
        u_view_proj: [[f32; 4]; 4],
        u_world: [[f32; 4]; 4],
    );
}
