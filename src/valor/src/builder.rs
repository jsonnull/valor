use glutin;
use gfx_window_glutin;
use gfx_text;
use Renderer;
use types::*;
use defines::pipe;

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

    pub fn finish(self) -> (glutin::GlWindow, glutin::EventsLoop, Renderer) {
        use gfx::traits::FactoryExt;

        // Prepare Glutin builders
        let events_loop = glutin::EventsLoop::new();
        let builder = glutin::WindowBuilder::new()
            .with_title(self.title)
            .with_dimensions(self.width, self.height);
        let context = glutin::ContextBuilder::new().with_vsync(self.vsync);

        // Create window- and context-dependent info
        let (window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop);

        // Create encoder, which will fill command buffer
        let encoder = factory.create_command_buffer().into();

        // Create text renderer
        let text = gfx_text::new(factory.clone()).build().unwrap();

        let pso_simple = factory
            .create_pipeline_simple(
                include_bytes!("shaders/triangle_150_vs.glsl"),
                include_bytes!("shaders/triangle_150_fs.glsl"),
                pipe::new(),
            )
            .unwrap();

        let globals = factory.create_constant_buffer(1);

        let (width, height) = match window.get_inner_size_pixels() {
            Some((w, h)) => (w, h),
            None => (0, 0),
        };

        let renderer = Renderer::new(
            factory,
            device,
            encoder,
            main_color,
            main_depth,
            self.clear_color,
            width,
            height,
            globals,
            text,
            pso_simple,
        );

        (window, events_loop, renderer)
    }
}
