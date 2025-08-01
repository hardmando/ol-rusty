use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{Window, WindowBuilder},
};

pub struct WindowHandle {
    pub event_loop: EventLoop<()>,
    pub window: Arc<Window>,
}

pub fn create_window(title: &str, width: u32, height: u32) -> WindowHandle {
    let event_loop = EventLoopBuilder::new().build();

    let window = Arc::new(
        WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap(),
    );

    WindowHandle { event_loop, window }
}

pub fn run_event_loop<F>(event_loop: EventLoop<()>, mut on_frame: F)
where
    F: 'static + FnMut(),
{
    event_loop.run(move |event, _target, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }

            Event::MainEventsCleared => {
                on_frame();
            }

            Event::RedrawRequested(_) => {
                println!("Redraw Request");
            }
            _ => {}
        }
    });
}
