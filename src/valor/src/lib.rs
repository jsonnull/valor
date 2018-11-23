//! A rendering library with scene-graph, asset management, and camera controls
//!
//! # Getting Started
//!
//! The first step for using Valor is to create the `Renderer` you'll work with.
//! Using the `ValorBuilder` struct is an easy way to create a `Renderer` and
//! window with common options for games.
//!
//! ```
//! # extern crate valor;
//! # use valor::ValorBuilder;
//! # pub fn main() {
//! let (mut events_loop, mut renderer) = ValorBuilder::new()
//!     .with_title(&"Minimal")
//!     .with_dimensions(800, 600)
//!     .finish();
//! # }
//! ```
//!
//! After you have a `Renderer`, there's some other things you can try:
//!
//!  - Draw some things using a `Material` and `Model`. One is provided by the
//!  `simple` module, but you can make your own.
//!  - Create a `Camera` and use it to move your view around
//!  - Create a `Scene` and traverse it when rendering

#![deny(missing_docs)]

pub use cgmath;
pub use glium;
pub use glutin;

use std::cell::RefCell;
use std::rc::Rc;

mod builder;
pub mod camera;
mod material;
mod renderer;
pub mod scene;
pub mod simple;
mod text;

pub use crate::builder::ValorBuilder;
pub use crate::material::Material;
pub use crate::renderer::Renderer;
pub use crate::text::{Text, TextHandle};

/// Utility type used when passing around models
pub type Handle<T> = Rc<RefCell<Box<T>>>;
