use id_tree::{Node, NodeId, Tree, TreeBuilder};
use cgmath::{Matrix4, One};

mod address;
mod node;
mod traversal;

pub use self::address::SceneAddress;
pub use self::node::{SceneNode, SceneNodeEntry};
pub use self::traversal::SceneTraversal;

/// A directed acyclic graph for models with transformations at each node.
pub struct Scene<T> {
    root_id: NodeId,
    pub graph: Tree<SceneNode<T>>,
}

impl<T> Scene<T> {
    pub fn new() -> Self {
        use id_tree::InsertBehavior::*;

        let mut graph: Tree<SceneNode<T>> = TreeBuilder::new().build();

        let root_node = SceneNode::empty();

        let root_id = graph.insert(Node::new(root_node), AsRoot).unwrap();

        Scene { root_id, graph }
    }

    pub fn traverse(&self) -> SceneTraversal<T> {
        let iter = self.graph.traverse_pre_order(&self.root_id).unwrap();
        SceneTraversal::new(iter)
    }

    /// Get ancestor transforms for a given node
    pub fn get_transform(&self, node: &SceneNode<T>, parent_id: Option<&NodeId>) -> Matrix4<f32> {
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

    pub fn insert(&mut self, node: SceneNode<T>, address: SceneAddress) -> NodeId {
        use id_tree::InsertBehavior::*;

        let location = match address {
            SceneAddress::Root => UnderNode(&self.root_id),
            SceneAddress::Parent(ref parent_id) => UnderNode(&parent_id),
        };

        let child_id = self.graph.insert(Node::new(node), location).unwrap();

        child_id
    }

    pub fn get_root_id(&self) -> NodeId {
        self.root_id.clone()
    }

    pub fn get_graph(&mut self) -> &mut Tree<SceneNode<T>> {
        &mut self.graph
    }
}
