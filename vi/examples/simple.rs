use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let mut display = sjvi::create_display(event_loop);

    while !display.should_close() {
        display.update(|| {});
    }
}
