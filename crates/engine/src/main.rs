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
    let r_window = window.clone();
    r_window.request_redraw(); // 👈 Explicit redraw request
    println!("Window created: {:?}", window.inner_size());
    ol_windowing::run_event_loop(event_loop, move || {
        r_window.request_redraw();
    });
    Ok(())
}
