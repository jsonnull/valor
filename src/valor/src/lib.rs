//! A rendering library with scene-graph, asset management, and camera controls
//!
//! ## Overview
//! The main entries are `Renderer`, `Scene`, and `Camera`.

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate gfx_text;
extern crate glutin;
extern crate cgmath;
extern crate id_tree;

mod builder;
mod camera;
pub(crate) mod defines;
mod model;
mod renderer;
mod scene;
mod text;
pub(crate) mod types;

pub use builder::ValorBuilder;
pub use camera::Camera;
pub use model::Model;
pub use model::ModelHandle;
pub use text::Text;
pub use text::TextHandle;
pub use renderer::Renderer;
pub use scene::SceneNode;
pub use scene::SceneNodeEntry;
pub use scene::Scene;
pub use defines::Vertex;

/// Used to switch between drawing pipelines for objects.
#[derive(Debug, PartialEq, Eq)]
pub enum RenderGroup {
    Simple,
    None,
}
