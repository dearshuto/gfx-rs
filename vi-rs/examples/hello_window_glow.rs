use glow::HasContext;

fn main() {
    let mut instance = sjvi::glutin::Instance::new();
    let id = instance.create_display();
    let display = instance.try_get_display(id).unwrap();
    let gl = unsafe { glow::Context::from_loader_function(|s| display.window.get_proc_address(s)) };

    while instance.try_update() {
        let display = instance.try_get_display(id).unwrap();
        if display.is_redraw_requested() {
            unsafe { gl.clear(glow::COLOR_BUFFER_BIT) }
            unsafe { gl.clear_color(0.1, 0.2, 0.3, 0.0) }
            display.window.swap_buffers().unwrap();
        }
    }
}
