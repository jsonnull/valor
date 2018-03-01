use glium;
use Handle;

pub trait Material<T> {
    fn draw(
        &self,
        target: &mut glium::Frame,
        model: Handle<T>,
        u_view_proj: [[f32; 4]; 4],
        u_world: [[f32; 4]; 4],
    );
}
