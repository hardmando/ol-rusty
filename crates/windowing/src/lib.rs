use winit::error::OsError;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

#[allow(deprecated)]
pub fn gen_window() -> Result<Window, OsError> {
    let event_loop = EventLoop::new().unwrap();
    let window_attributes = Window::default_attributes()
        .with_title("ol-rusty")
        .with_visible(true)
        .with_active(true);
    event_loop.create_window(window_attributes)
}
