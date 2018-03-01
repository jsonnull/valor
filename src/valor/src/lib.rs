//! A rendering library with scene-graph, asset management, and camera controls
//!
//! ## Overview
//! The main entries are `Renderer`, `Scene`, and `Camera`.

extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate id_tree;

use std::rc::Rc;
use std::cell::RefCell;

mod builder;
mod camera;
mod renderer;
mod scene;
mod text;
mod material;
pub mod simple;

pub use builder::ValorBuilder;
pub use camera::Camera;
pub use text::Text;
pub use text::TextHandle;
pub use renderer::Renderer;
pub use scene::SceneNode;
pub use scene::SceneNodeEntry;
pub use scene::Scene;
pub use material::Material;

pub type Handle<T> = Rc<RefCell<Box<T>>>;
