use id_tree::NodeId;

/// Disambiguate between the root node of the scene graph and an arbitrary
/// address.
#[derive(Clone)]
pub enum Address {
    /// Encode a specific `NodeId`
    Parent(NodeId),
    /// Refer to the scene root specifically
    Root,
}
