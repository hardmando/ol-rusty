fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ol_windowing::WindowHandle { event_loop, window } =
        ol_windowing::create_window("ol-rusty", 1280, 720);

    let renderer = match ol_renderer::Renderer::new(&window) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to create renderer: {:?}", e);
            return Ok(());
        }
    };
    
    println!("Window created: {:?}", window.inner_size());
    ol_windowing::run_event_loop(event_loop, move || {
        // This will be called when redraw is requested
        println!("Rendering frame...");
    });
    Ok(())
}
