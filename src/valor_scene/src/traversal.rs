use crate::Node;
use cgmath::{Matrix4, One};
use generational_arena::{Arena, Index};

type Transform = Matrix4<f32>;

#[derive(Debug)]
enum TraversalAction {
  Entry(Index),
  Descend(Index),
  Ascend,
}

/// `Iterator` over `Node`s in a `Scene`.
pub struct Traversal<'a, T> {
  /// Stack of transforms
  transforms: Vec<Transform>,
  /// Action
  action_stream: Vec<TraversalAction>,
  // Pool of nodes to retrieve from
  arena: &'a Arena<Node<T>>,
}

impl<'a, T> Traversal<'a, T> {
  /// Create a new new traversal over children of a given node
  pub fn new(arena: &'a Arena<Node<T>>, root: Index) -> Self {
    // Stack begins with the root element's ID and transform
    let transforms = vec![Matrix4::one()];
    let action_stream = vec![TraversalAction::Entry(root)];

    Traversal {
      transforms,
      action_stream,
      arena,
    }
  }

  pub fn add_children(&mut self, index: Index, children: &Vec<Index>) {
    let mut actions = children
      .iter()
      .cloned()
      .map(|index| TraversalAction::Entry(index))
      .collect();

    self.action_stream.push(TraversalAction::Ascend);
    self.action_stream.append(&mut actions);
    self.action_stream.push(TraversalAction::Descend(index));
  }
}

impl<'a, T> Iterator for Traversal<'a, T> {
  type Item = (Index, &'a T, Transform);

  fn next(&mut self) -> Option<Self::Item> {
    match self.action_stream.pop() {
      Some(action) => match action {
        TraversalAction::Entry(index) => {
          if let Some(node) = self.arena.get(index) {
            let children = node.children.borrow();

            // If this node has children, process them next
            if !children.is_empty() {
              self.add_children(index, &children);
            };

            // Calculate ancestor transform times this node's transform
            let transform = match self.transforms.last() {
              Some(t) => t * node.transform,
              None => node.transform,
            };

            let data = &node.data;

            Some((index, data, transform))
          } else {
            self.next()
          }
        }
        TraversalAction::Descend(index) => {
          if let Some(node) = self.arena.get(index) {
            // Calculate cumulative transform for ancestors through the parent
            let transform = match self.transforms.last() {
              Some(t) => t * node.transform,
              None => node.transform,
            };

            // Add it to the stack
            self.transforms.push(transform);
          }

          // Continue iteration
          self.next()
        }
        TraversalAction::Ascend => {
          // Remove the parent transform
          self.transforms.pop();
          self.next()
        }
      },
      None => None,
    }
  }
}
