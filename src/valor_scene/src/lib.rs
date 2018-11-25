//! An easy way to create, manipulate, and traverse component trees.
mod node;
mod traversal;

pub use crate::node::Node;
pub use crate::traversal::Traversal;
use cgmath::Vector3;
use generational_arena::{Arena, Index};

/// A directed acyclic graph for models with transformations at each node.
pub struct Scene<T> {
    arena: Arena<Node<T>>,
    root: Index,
}

impl<T> Scene<T> {
    /// Creates a new scene and inserts an empty root node
    pub fn new(root: T) -> Self {
        let mut arena = Arena::new();
        let root = arena.insert(Node::new(root));
        Scene { arena, root }
    }

    pub fn create_node(&mut self, data: T) -> Index {
        let node = Node::new(data);
        let index = self.arena.insert(node);
        index
    }

    pub fn get_root(&self) -> Index {
        self.root
    }

    pub fn translate(&mut self, id: Index, translation: Vector3<f32>) {
        if let Some(node) = self.arena.get_mut(id) {
            node.translate(translation);
        }
    }

    pub fn add_child(&mut self, parent: Index, child: Index) {
        if let Some(node) = self.arena.get_mut(parent) {
            node.add_child(child);
        }
    }

    pub fn get(&self, id: Index) -> Option<&Node<T>> {
        self.arena.get(id)
    }

    pub fn get_mut(&mut self, id: Index) -> Option<&mut Node<T>> {
        self.arena.get_mut(id)
    }

    pub fn traverse(&self) -> Traversal<T> {
        Traversal::new(&self.arena, self.root)
    }
}
