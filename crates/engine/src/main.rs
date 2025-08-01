fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting ol-rusty engine...");
    
    let ol_windowing::WindowHandle { event_loop, window } =
        ol_windowing::create_window("ol-rusty", 1280, 720);

    println!("Window created, skipping renderer for now...");
    
    // Don't create renderer yet - just test window visibility
    println!("Engine initialized successfully, starting event loop...");
    
    // Keep the window alive for a while
    let start = std::time::Instant::now();
    
    ol_windowing::run_event_loop(event_loop, move || {
        // This will be called when redraw is requested
        println!("Rendering frame... (elapsed: {}s)", start.elapsed().as_secs());
        
        // Exit after 30 seconds if window doesn't close
        if start.elapsed().as_secs() > 30 {
            println!("Exiting after 30 seconds");
            std::process::exit(0);
        }
    });
    
    println!("Event loop ended");
    Ok(())
}
