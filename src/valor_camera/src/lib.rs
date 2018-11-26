//! Provides an ergonomic abstraction for a View-Projection matrix.
//!
//! # Examples
//!
//! Create a camera:
//!
//! ```
//! use valor::camera::Camera;
//!
//! let mut camera = Camera::new();
//! ```

mod builder;

pub use crate::builder::CameraBuilder;
use cgmath::{perspective, Deg, Matrix4, Vector3};

/// Perspective camera with positioning controls.
///
/// # Example usage
///
/// ```
/// # use valor::camera::Camera;
/// let mut camera = Camera::new();
///
/// let view_proj_matrix: [[f32; 4]; 4] = camera.get_view_proj().into();
/// ```
pub struct Camera {
    position: Vector3<f32>,
    perspective: Matrix4<f32>,
    pitch: f32,
    yaw: f32,
    sensitivity: f32,
}

impl Camera {
    /// Create a new camera with some default attributes
    pub fn new(perspective: Matrix4<f32>, sensitivity: f32) -> Self {
        let position = Vector3::new(0.0, 0.0, 0.0);

        Camera {
            position,
            perspective,
            sensitivity,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn use_perspective(&mut self, fov: f32, aspect: f32, near: f32, far: f32) {
        self.perspective = perspective(Deg(fov), aspect, near, far);
    }

    pub fn use_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = sensitivity;
    }

    /// Calculate the combined transformation matrix encoded by the `Camera`
    pub fn get_view_proj(&self) -> Matrix4<f32> {
        let rotation =
            Matrix4::from_angle_x(Deg(self.pitch)) * Matrix4::from_angle_y(Deg(self.yaw));

        let translation = Matrix4::from_translation(self.position);

        self.perspective * rotation * translation
    }

    /// Update the pitch and yaw attributes of the camera as a mouse moves
    /// `pitch_delta` and `yaw_delta` are pixel movement distances
    pub fn mouse_look(&mut self, pitch_delta: f32, yaw_delta: f32) {
        let mut pitch = self.pitch + (pitch_delta * self.sensitivity);
        let yaw = self.yaw + (yaw_delta * self.sensitivity);

        // Do not let the player look backwards up or down
        if pitch < -90.0 {
            pitch = -90.0
        } else if pitch > 90.0 {
            pitch = 90.0
        }

        self.pitch = pitch;
        self.yaw = yaw;
    }
}
