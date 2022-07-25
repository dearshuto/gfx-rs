use gfx_websys::DeviceWsys;
use sjgfx_interface::{DeviceInfo, IDevice};

use web_sys::WebGl2RenderingContext as GL;

fn main() {
    let mut instance = sjvi::web_sys::Instance::new();
    let id = instance.create_display();
    let display = instance.try_get_display(&id).unwrap();

    let device = DeviceWsys::new_with_surface(&DeviceInfo::new(), &display);

    let gl = device.clone_context();
    gl.clear_color(0.1, 0.2, 0.3, 1.0);
    gl.clear(GL::COLOR_BUFFER_BIT);
}
