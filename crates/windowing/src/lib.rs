use winit::{
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

pub struct WindowBuilder {
    title: String,
    width: u32,
    height: u32,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            title: "ol-rusty Engine".to_string(),
            width: 800,
            height: 600,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn build(self) -> Result<(EventLoop<()>, Window), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;

        let window_attributes = WindowAttributes::default()
            .with_title(self.title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.width as f64,
                self.height as f64,
            ));

        let window = event_loop.create_window(window_attributes)?;

        Ok((event_loop, window))
    }
}

