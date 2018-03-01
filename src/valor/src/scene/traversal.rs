use id_tree::*;
use super::SceneNode;

pub struct SceneTraversal<'a, T: 'a> {
    iter: PreOrderTraversal<'a, SceneNode<T>>,
}

impl<'a, T> SceneTraversal<'a, T> {
    pub fn new(iter: PreOrderTraversal<'a, SceneNode<T>>) -> Self {
        SceneTraversal { iter: iter }
    }
}

type SceneTuple<'a, T> = (&'a SceneNode<T>, Option<&'a NodeId>);
impl<'a, T: 'a> Iterator for SceneTraversal<'a, T> {
    type Item = SceneTuple<'a, T>;

    fn next(&mut self) -> Option<SceneTuple<'a, T>> {
        match self.iter.next() {
            Some(node_container) => {
                let node: &SceneNode<T> = node_container.data();
                let parent_id = node_container.parent();
                Some((node, parent_id))
            }
            None => None,
        }
    }
}
