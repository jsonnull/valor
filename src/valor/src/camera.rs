use cgmath::{perspective, Deg, Matrix4, Vector3};

/// Perspective camera with positioning controls.
pub struct Camera {
    position: Vector3<f32>,
    perspective: Matrix4<f32>,
    pitch: f32,
    yaw: f32,
}

const SENSITIVITY: f32 = 1.3 / 20.0;
const FOV: f32 = 90.0;

impl Camera {
    pub fn new() -> Self {
        let position = Vector3::new(0.0, 0.0, 0.0);

        let aspect = 4.0f32 / 3.0f32;
        let perspective = perspective(Deg(FOV), aspect, 0.1, 50.0);

        Camera {
            position,
            perspective,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn get_view_proj(&self) -> Matrix4<f32> {
        let rotation =
            Matrix4::from_angle_x(Deg(self.pitch)) * Matrix4::from_angle_y(Deg(self.yaw));

        let translation = Matrix4::from_translation(self.position);

        self.perspective * rotation * translation
    }

    /// `pitch_delta` and `yaw_delta` are pixel movement distances
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
