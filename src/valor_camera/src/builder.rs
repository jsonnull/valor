use crate::Camera;
use cgmath::{perspective, Deg};

const DEFAULT_FOV: f32 = 80.0;
const DEFAULT_ASPECT: f32 = 4.0 / 3.0;
const DEFAULT_NEAR: f32 = 0.1;
const DEFAULT_FAR: f32 = 50.0;
const DEFAULT_SENSITIVITY: f32 = 1.3 / 20.0;

pub struct CameraBuilder {
  fov: f32,
  aspect_ratio: f32,
  near_plane: f32,
  far_plane: f32,
  sensitivity: f32,
}

impl CameraBuilder {
  pub fn new() -> Self {
    CameraBuilder {
      fov: DEFAULT_FOV,
      aspect_ratio: DEFAULT_ASPECT,
      near_plane: DEFAULT_NEAR,
      far_plane: DEFAULT_FAR,
      sensitivity: DEFAULT_SENSITIVITY,
    }
  }

  pub fn with_fov(mut self, fov: f32) -> Self {
    self.fov = fov;
    self
  }

  pub fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
    self.aspect_ratio = aspect_ratio;
    self
  }

  pub fn with_near_plane(mut self, near_plane: f32) -> Self {
    self.near_plane = near_plane;
    self
  }

  pub fn with_far_plane(mut self, far_plane: f32) -> Self {
    self.far_plane = far_plane;
    self
  }

  pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
    self.sensitivity = sensitivity;
    self
  }

  pub fn finish(self) -> Camera {
    let perspective = perspective(
      Deg(self.fov),
      self.aspect_ratio,
      self.near_plane,
      self.far_plane,
    );

    Camera::new(perspective, self.sensitivity)
  }
}
