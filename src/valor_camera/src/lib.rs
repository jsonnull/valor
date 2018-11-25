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
}

const SENSITIVITY: f32 = 1.3 / 20.0;
const FOV: f32 = 90.0;

impl Camera {
    /// Create a new camera with some default attributes
    pub fn new() -> Self {
        let position = Vector3::new(0.0, 0.0, 0.0);

        let aspect = 4.0 / 3.0;
        let perspective = perspective(Deg(FOV), aspect, 0.1, 50.0);

        Camera {
            position,
            perspective,
            pitch: 0.0,
            yaw: 0.0,
        }
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
    // TODO: Add make sensitivity configurable
    pub fn mouse_look(&mut self, pitch_delta: f32, yaw_delta: f32) {
        let mut pitch = self.pitch + (pitch_delta * SENSITIVITY);
        let yaw = self.yaw + (yaw_delta * SENSITIVITY);

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