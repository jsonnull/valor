use cgmath::{Matrix4, One, Vector3};

/// Enum value which encapsulates all scene node types.
pub enum NodeEntry<T> {
    /// A model is located at this node
    Model(T),
    // Text(TextHandle),
    /// This node has no renderable content, but has a transform and may be
    /// parent to other nodes.
    Empty,
}

/// Scene entries have an optional model and a transform.
pub struct Node<T> {
    /// The value stored at this node
    pub entry: NodeEntry<T>,
    /// The transform matrix of this node
    pub transform: Matrix4<f32>,
}

impl<T> Node<T> {
    /// Construct a new scene node which holds the given entry
    pub fn new(entry: NodeEntry<T>) -> Self {
        Node {
            entry: entry,
            transform: Matrix4::one(),
        }
    }

    /// Construct an scene node with no value, but has a transform and may be
    /// parent to other nodes
    pub fn empty() -> Self {
        Node {
            entry: NodeEntry::Empty,
            transform: Matrix4::one(),
        }
    }

    /// Perform a translation on this node, updating the underlying transform
    /// matrix
    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.transform = self.transform * Matrix4::from_translation(translation);
    }
}
