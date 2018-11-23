//! Implements a basic material you can use.
mod material;
mod model;
mod vertex;

pub use self::material::SimpleMaterial as Material;
pub use self::model::Model;
pub use self::vertex::Vertex;
