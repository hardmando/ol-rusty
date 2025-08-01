fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ol_windowing::WindowHandle { event_loop, window } =
        ol_windowing::create_window("ol-rusty", 1280, 720);

    println!("Window created: {:?}", window.inner_size());
    
    // Try to create renderer with better error reporting
    match ol_renderer::Renderer::new(&window) {
        Ok(renderer) => {
            println!("Renderer created successfully!");
            ol_windowing::run_event_loop(event_loop, move || {
                println!("Rendering frame...");
            });
        }
        Err(e) => {
            eprintln!("Failed to create renderer: {:?}", e);
            eprintln!("This is likely a Vulkan driver compatibility issue.");
            eprintln!("Try running with X11: WAYLAND_DISPLAY= DISPLAY=:0 cargo run -p ol_engine");
            return Ok(());
        }
    };
    
    Ok(())
}
