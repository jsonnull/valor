use crate::Renderer;
use glium;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// Configuration object for initializing `Renderer` and window systems.
///
/// # Example
///
/// ```
/// use valor::ValorBuilder;
///
/// const LIGHT_BLUE: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
///
/// let (mut events_loop, mut renderer) = ValorBuilder::new()
///     .with_title(&"Basic example")
///     .with_dimensions(800, 600)
///     .with_clear_color(LIGHT_BLUE)
///     .with_vsync(true)
///     .finish();
/// ```
pub struct ValorBuilder {
    title: &'static str,
    width: f64,
    height: f64,
    vsync: bool,
    clear_color: [f32; 4],
}

impl ValorBuilder {
    /// Construct a new builder instance with default values
    pub fn new() -> Self {
        ValorBuilder {
            title: "Valor",
            width: 640.0,
            height: 480.0,
            vsync: false,
            clear_color: BLACK,
        }
    }

    /// Update the resulting window title
    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    /// Update the starting dimensions of the window
    pub fn with_dimensions(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Update whether the resulting window should use vsync
    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    /// Update the clear color used at the beginning of each frame draw the
    /// `Renderer` performs
    pub fn with_clear_color(mut self, clear_color: [f32; 4]) -> Self {
        self.clear_color = clear_color;
        self
    }

    /// Complete the build
    pub fn finish(self) -> (glium::glutin::EventsLoop, Renderer) {
        // Prepare Glutin builders
        let events_loop = glium::glutin::EventsLoop::new();
        let builder = glium::glutin::WindowBuilder::new()
            .with_title(self.title)
            .with_dimensions(glium::glutin::dpi::LogicalSize::new(
                self.width,
                self.height,
            ));
        let context = glium::glutin::ContextBuilder::new().with_vsync(self.vsync);

        let window = glium::Display::new(builder, context, &events_loop).unwrap();

        let (width, height): (u32, u32) = match window.gl_window().get_inner_size() {
            Some(size) => size.into(),
            None => (0, 0),
        };

        let renderer = Renderer::new(window, self.clear_color, width, height);

        (events_loop, renderer)
    }
}
