fn main() {
    let ol_windowing::WindowHandle { event_loop, .. } =
        ol_windowing::create_window("ol-rusty", 1280, 720);

    ol_windowing::run_event_loop(event_loop, || {
        // placeholder for drawing/updating
    });
}
