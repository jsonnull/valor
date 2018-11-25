use cgmath::{Matrix4, One, Vector3};
use generational_arena::Index;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Node<T> {
  pub data: T,
  pub children: RefCell<Vec<Index>>,
  pub transform: Matrix4<f32>,
}

impl<T> Node<T> {
  pub fn new(data: T) -> Self {
    let children = RefCell::from(vec![]);
    let transform = Matrix4::one();

    Node {
      data,
      children,
      transform,
    }
  }

  pub fn add_child(&self, child: Index) -> &Self {
    let mut children = self.children.borrow_mut();
    children.push(child);
    self
  }

  /// Perform a translation on this node, updating the underlying transform
  /// matrix
  pub fn translate(&mut self, translation: Vector3<f32>) {
    self.transform = self.transform * Matrix4::from_translation(translation);
  }
}
