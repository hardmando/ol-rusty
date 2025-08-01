fn main() {
    let ol_windowing::WindowHandle { event_loop, window } =
        ol_windowing::create_window("ol-rusty", 1280, 720);

    let r_window = window.clone();
    ol_windowing::run_event_loop(event_loop, move || {
        println!("frame!");
        r_window.request_redraw();
    });
}
