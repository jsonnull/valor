#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 4], //  = "a_Pos",
    color: [f32; 3],    // = "a_Color",
}

implement_vertex!(Vertex, position, color);

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, color: [f32; 3]) -> Self {
        Vertex {
            position: [x, y, z, 1.0],
            color,
        }
    }
}
