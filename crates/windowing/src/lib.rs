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

    let window = match WindowBuilder::new()
        .with_visible(true)
        .with_title(title)
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .with_position(winit::dpi::LogicalPosition::new(100.0, 100.0))
        .with_decorations(true)
        .with_resizable(true)
        .build(&event_loop)
    {
        Ok(window) => {
            println!("Window created successfully: {:?}", window.inner_size());
            window.set_visible(true);
            window.focus_window();
            window.request_redraw();
            Arc::new(window)
        }
        Err(e) => {
            eprintln!("Failed to create window: {:?}", e);
            panic!("Window creation failed");
        }
    };

    WindowHandle { event_loop, window }
}

pub fn run_event_loop<F: 'static + FnMut()>(event_loop: EventLoop<()>, mut update: F) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(size) => {
                        println!("Window resized to: {:?}", size);
                    }
                    WindowEvent::Focused(focused) => {
                        println!("Window focused: {}", focused);
                    }
                    _ => {}
                }
            }

            Event::RedrawRequested(_) => {
                update();
            }

            _ => {}
        }
    });
}
