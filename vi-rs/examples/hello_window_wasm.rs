use glow::HasContext;

fn main() {
    let mut instance = sjvi::web_sys::Instance::new();
    let id = instance.create_display();
    let display = instance.try_get_display(&id).unwrap();
    let gl = glow::Context::from_webgl2_context(display.context.clone());

    unsafe {
        gl.clear_color(0.6, 0.2, 0.3, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);
    }
}
