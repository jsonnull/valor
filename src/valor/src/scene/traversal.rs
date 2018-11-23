use super::Node;
use id_tree::*;

/// `Iterator` over `Node`s in a `Scene`.
pub struct Traversal<'a, T: 'a> {
    iter: PreOrderTraversal<'a, Node<T>>,
}

impl<'a, T> Traversal<'a, T> {
    /// Create a new new traversal from the `id_tree`'s traversal.
    pub fn new(iter: PreOrderTraversal<'a, Node<T>>) -> Self {
        Traversal { iter }
    }
}

type SceneTuple<'a, T> = (&'a Node<T>, Option<&'a NodeId>);
impl<'a, T: 'a> Iterator for Traversal<'a, T> {
    type Item = SceneTuple<'a, T>;

    fn next(&mut self) -> Option<SceneTuple<'a, T>> {
        match self.iter.next() {
            Some(node_container) => {
                let node: &Node<T> = node_container.data();
                let parent_id = node_container.parent();
                Some((node, parent_id))
            }
            None => None,
        }
    }
}
