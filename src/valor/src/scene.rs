use cgmath::{Matrix4, Vector3, One};
use id_tree::*;
use ModelHandle;
use TextHandle;
use RenderGroup;

pub enum SceneNodeEntry {
    Model(ModelHandle),
    Text(TextHandle),
    Empty,
}

/// Scene entries have an optional model and a transform.
pub struct SceneNode {
    pub entry: SceneNodeEntry,
    pub group: RenderGroup,
    pub transform: Matrix4<f32>,
}

impl SceneNode {
    pub fn new(entry: SceneNodeEntry) -> Self {
        SceneNode {
            entry: entry,
            group: RenderGroup::Simple,
            transform: Matrix4::one(),
        }
    }

    pub fn empty() -> Self {
        SceneNode {
            entry: SceneNodeEntry::Empty,
            group: RenderGroup::None,
            transform: Matrix4::one(),
        }
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.transform = self.transform * Matrix4::from_translation(translation);
    }
}

/// A directed acyclic graph for models with transformations at each node.
pub struct Scene {
    root_id: NodeId,
    pub graph: Tree<SceneNode>,
}

impl Scene {
    pub fn new() -> Self {
        use id_tree::InsertBehavior::*;

        let mut graph: Tree<SceneNode> = TreeBuilder::new().build();

        let root_node = SceneNode::empty();

        let root_id = graph.insert(Node::new(root_node), AsRoot).unwrap();

        Scene { root_id, graph }
    }

    pub fn traverse(&self) -> PreOrderTraversal<SceneNode> {
        self.graph.traverse_pre_order(&self.root_id).unwrap()
    }

    pub fn insert(&mut self, node: SceneNode, parent_id: &NodeId) -> NodeId {
        use id_tree::InsertBehavior::*;

        let child_id = self.graph
            .insert(Node::new(node), UnderNode(parent_id))
            .unwrap();

        child_id
    }

    pub fn get_root_id(&self) -> NodeId {
        self.root_id.clone()
    }

    pub fn get_graph(&mut self) -> &mut Tree<SceneNode> {
        &mut self.graph
    }
}
