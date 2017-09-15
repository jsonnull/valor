use gfx;
use gfx_device_gl;

// Type aliases to make the Renderer code a bit easier to work with

pub type Resources = gfx_device_gl::Resources;
pub type CommandBuffer = gfx_device_gl::CommandBuffer;
pub type Encoder = gfx::Encoder<Resources, CommandBuffer>;
pub type Device = gfx_device_gl::Device;
pub type Factory = gfx_device_gl::Factory;

pub type ColorFormat = gfx::format::Rgba8;
//pub type NormalFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;
pub type OutputColor = gfx::handle::RenderTargetView<Resources, ColorFormat>;
pub type OutputDepth = gfx::handle::DepthStencilView<Resources, DepthFormat>;

pub type PipelineState<T> = gfx::pso::PipelineState<Resources, T>;
//pub type PipelineData<D> = gfx::pso::PipelineData<Resources, Meta = D>;
//pub type Slice = gfx::Slice<Resources>;
//pub type Buffer<T> = gfx::handle::Buffer<Resources, T>;
