use glium;
use Renderer;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// Configuration object for initializing `Renderer` and window systems.
pub struct ValorBuilder {
    title: &'static str,
    width: u32,
    height: u32,
    vsync: bool,
    clear_color: [f32; 4],
}

impl ValorBuilder {
    pub fn new() -> Self {
        ValorBuilder {
            title: "Valor",
            width: 640,
            height: 480,
            vsync: false,
            clear_color: BLACK,
        }
    }

    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn with_clear_color(mut self, clear_color: [f32; 4]) -> Self {
        self.clear_color = clear_color;
        self
    }

    pub fn finish(self) -> (glium::glutin::EventsLoop, Renderer) {
        // Prepare Glutin builders
        let events_loop = glium::glutin::EventsLoop::new();
        let builder = glium::glutin::WindowBuilder::new()
            .with_title(self.title)
            .with_dimensions(self.width, self.height);
        let context = glium::glutin::ContextBuilder::new().with_vsync(self.vsync);

        let window = glium::Display::new(builder, context, &events_loop).unwrap();

        let (width, height) = match window.gl_window().get_inner_size_pixels() {
            Some((w, h)) => (w, h),
            None => (0, 0),
        };

        let renderer = Renderer::new(window, self.clear_color, width, height);

        (events_loop, renderer)
    }
}
