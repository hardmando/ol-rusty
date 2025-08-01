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
    println!("Creating window with title: {}, size: {}x{}", title, width, height);
    
    let event_loop = EventLoopBuilder::new().build().expect("Failed to create event loop");
    println!("Event loop created successfully");

    let window = match WindowBuilder::new()
        .with_visible(true)
        .with_title(title)
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .with_position(winit::dpi::LogicalPosition::new(100.0, 100.0)) // Position window at (100, 100)
        .with_decorations(true) // Ensure window has decorations
        .with_resizable(true)
        .with_transparent(false) // Ensure window is not transparent
        .build(&event_loop)
    {
        Ok(window) => {
            println!("Window created successfully: {:?}", window.inner_size());
            println!("Window position: {:?}", window.outer_position());
            println!("Window is visible: {}", window.is_visible().unwrap_or(false));
            
            // Force window to be visible and focused
            window.set_visible(true);
            window.focus_window();
            
            // Request immediate redraw to ensure window appears
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
    println!("Starting event loop...");
    event_loop.run(move |event, target| {
        target.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("Window close requested");
                        target.exit();
                    }
                    WindowEvent::Resized(size) => {
                        println!("Window resized to: {:?}", size);
                    }
                    WindowEvent::Moved(position) => {
                        println!("Window moved to: {:?}", position);
                    }
                    WindowEvent::Focused(focused) => {
                        println!("Window focused: {}", focused);
                    }
                    WindowEvent::CursorEntered { .. } => {
                        println!("Cursor entered window");
                    }
                    WindowEvent::CursorLeft { .. } => {
                        println!("Cursor left window");
                    }
                    WindowEvent::MouseInput { state, .. } => {
                        println!("Mouse input: {:?}", state);
                    }
                    _ => {}
                }
            }

            Event::AboutToWait => {
                update();
                println!("Redraw requested");
            }

            Event::DeviceEvent { .. } => {
                println!("Device event received");
            }

            _ => {}
        }
    }).expect("Event loop failed");
}

// Simple test function to verify window creation
pub fn test_window_creation() {
    println!("Testing window creation...");
    
    // Try creating window with minimal configuration
    let event_loop = EventLoopBuilder::new().build().expect("Failed to create event loop");
    println!("Event loop created successfully");

    let window = match WindowBuilder::new()
        .with_title("HYPRLAND TEST WINDOW")
        .with_inner_size(winit::dpi::LogicalSize::new(400, 300))
        .with_visible(true)
        .build(&event_loop)
    {
        Ok(window) => {
            println!("Window created successfully: {:?}", window.inner_size());
            println!("Window is visible: {}", window.is_visible().unwrap_or(false));
            
            // Try to force visibility
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

    println!("Test window created successfully");
    
    // Run for a longer time to verify it works
    let start = std::time::Instant::now();
    let window_clone = window.clone();
    
    event_loop.run(move |event, target| {
        target.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("Window close requested");
                        target.exit();
                    }
                    WindowEvent::Resized(size) => {
                        println!("Window resized to: {:?}", size);
                    }
                    WindowEvent::Focused(focused) => {
                        println!("Window focused: {}", focused);
                        if !focused {
                            // Try to focus again
                            window_clone.focus_window();
                        }
                    }
                    WindowEvent::Moved(position) => {
                        println!("Window moved to: {:?}", position);
                    }
                    _ => {}
                }
            }

            Event::AboutToWait => {
                println!("Redraw requested (elapsed: {}s)", start.elapsed().as_secs());
                
                // Try to keep window visible
                window_clone.set_visible(true);
                window_clone.request_redraw();
                
                // Exit after 10 seconds if window doesn't close
                if start.elapsed().as_secs() > 10 {
                    println!("Test completed after 10 seconds");
                    std::process::exit(0);
                }
            }

            _ => {}
        }
    }).expect("Event loop failed");
}
