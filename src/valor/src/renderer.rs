use glium;

/// Interface responsible for window creation and initiating drawing.
pub struct Renderer {
    /// The underlying `glium::Display` we abstract over.
    pub display: glium::Display,

    /// Color for OpenGL clear command.
    clear_color: [f32; 4],

    /// Width of the frame.
    pub width: u32,
    /// Height of the frame.
    pub height: u32,
}

impl Renderer {
    /// Create a new rendering instance. You should probably use `ValorBuilder`
    /// instead of this method.
    pub(crate) fn new(
        display: glium::Display,
        clear_color: [f32; 4],
        width: u32,
        height: u32,
    ) -> Self {
        Renderer {
            display,
            clear_color,
            width,
            height,
        }
    }

    /// Setter for the frame dimensions, as these can change over the course of
    /// a window's lifetime.
    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Initiate a frame draw. This will begin the frame and call `callback`
    /// when the frame is ready to be drawn to.
    pub fn render<F>(&mut self, callback: F)
    where
        F: Fn(&mut glium::Frame),
    {
        use glium::Surface;

        let mut target = self.display.draw();

        // Clear background color
        target.clear_color(
            self.clear_color[0],
            self.clear_color[1],
            self.clear_color[2],
            self.clear_color[3],
        );

        callback(&mut target);

        target.finish().unwrap();
    }
}
