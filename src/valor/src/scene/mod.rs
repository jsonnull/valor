//! An easy way to create, manipulate, and traverse component trees.

use id_tree::{Node as IdTreeNode, NodeId, Tree, TreeBuilder};
use cgmath::{Matrix4, One};

mod address;
mod node;
mod traversal;

pub use self::address::Address;
pub use self::node::{Node, NodeEntry};
pub use self::traversal::Traversal;

/// A directed acyclic graph for models with transformations at each node.
pub struct Scene<T> {
    root_id: NodeId,
    graph: Tree<Node<T>>,
}

impl<T> Scene<T> {
    /// Creates a new scene and inserts an empty root node
    pub fn new() -> Self {
        use id_tree::InsertBehavior::*;

        let mut graph: Tree<Node<T>> = TreeBuilder::new().build();

        let root_node = Node::empty();

        let root_id = graph.insert(IdTreeNode::new(root_node), AsRoot).unwrap();

        Scene { root_id, graph }
    }

    /// Create a traversal iterator
    pub fn traverse(&self) -> Traversal<T> {
        let iter = self.graph.traverse_pre_order(&self.root_id).unwrap();
        Traversal::new(iter)
    }

    /// Get ancestor transforms for a given node
    pub fn get_transform(&self, node: &Node<T>, parent_id: Option<&NodeId>) -> Matrix4<f32> {
        let identity: Matrix4<f32> = Matrix4::one();

        // Transform accumulator
        let mut transforms: Vec<Matrix4<f32>> = vec![];

        // Pool transforms for ancestors of the node
        if let Some(id) = parent_id {
            // Get ancestor transforms
            transforms = self.graph
                .ancestors(id)
                .unwrap()
                .map(|n| n.data().transform)
                .collect();

            transforms.reverse();

            // Get parent transform
            let parent = self.graph.get(id).unwrap();
            transforms.push(parent.data().transform);
        };

        // Push this node's transform
        transforms.push(node.transform);

        // Multiply transforms out
        let transform = transforms.iter().fold(identity, |acc, t| acc * t);

        transform
    }

    /// Insert an item into the scene at a given location
    pub fn insert(&mut self, node: Node<T>, address: Address) -> NodeId {
        use id_tree::InsertBehavior::*;

        let location = match address {
            Address::Root => UnderNode(&self.root_id),
            Address::Parent(ref parent_id) => UnderNode(&parent_id),
        };

        let child_id = self.graph.insert(IdTreeNode::new(node), location).unwrap();

        child_id
    }

    /// Get the `NodeId` of the root node in the scene
    pub fn get_root_id(&self) -> NodeId {
        self.root_id.clone()
    }

    /// Retrieve the underlying data structure
    pub fn get_graph(&mut self) -> &mut Tree<Node<T>> {
        &mut self.graph
    }
}
