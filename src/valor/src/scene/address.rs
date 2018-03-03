use id_tree::NodeId;

/// Enum value which encapsulates all scene node types
pub enum SceneAddress {
    Parent(NodeId),
    Root,
}
