use cgmath::{Matrix4, One, Vector3};

/// Enum value which encapsulates all scene node types
pub enum SceneNodeEntry<T> {
    Model(T),
    // Text(TextHandle),
    Empty,
}

/// Scene entries have an optional model and a transform.
pub struct SceneNode<T> {
    pub entry: SceneNodeEntry<T>,
    pub transform: Matrix4<f32>,
}

impl<T> SceneNode<T> {
    pub fn new(entry: SceneNodeEntry<T>) -> Self {
        SceneNode {
            entry: entry,
            transform: Matrix4::one(),
        }
    }

    pub fn empty() -> Self {
        SceneNode {
            entry: SceneNodeEntry::Empty,
            transform: Matrix4::one(),
        }
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.transform = self.transform * Matrix4::from_translation(translation);
    }
}
