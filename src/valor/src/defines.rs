use gfx;
use types::*;

gfx_defines!{
    /// Basic model building block.
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Locals {
        mx_world: [[f32; 4]; 4] = "u_World",
        // color: [f32; 4] = "u_Color",
    }

    constant Globals {
        mx_vp: [[f32; 4]; 4] = "u_ViewProj",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        cb_locals: gfx::ConstantBuffer<Locals> = "b_Locals",
        cb_globals: gfx::ConstantBuffer<Globals> = "b_Globals",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, color: [f32; 3]) -> Self {
        Vertex {
            pos: [x, y, z, 1.0],
            color,
        }
    }
}

pub struct GpuData {
    pub slice: gfx::Slice<Resources>,
    pub vertices: gfx::handle::Buffer<Resources, Vertex>,
    pub locals: gfx::handle::Buffer<Resources, Locals>,
}
