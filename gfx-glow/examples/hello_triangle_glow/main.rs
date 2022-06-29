use glow::HasContext;
use sjgfx_interface::DeviceInfo;

fn main() {
    let vertex_shader_source = include_str!("resources/hello_triangle.vs");
    let pixel_shader_source = include_str!("resources/hello_triangle.fs");
    let device = sjgfx_glow::DeviceGlow::new(&DeviceInfo::new());
    let shader = sjgfx_glow::ShaderGlow::new(
        &device,
        &sjgfx_interface::ShaderInfo::new()
            .set_vertex_shader_source(vertex_shader_source)
            .set_pixel_shader_source(pixel_shader_source),
    );

    let context = device.clone_context();
    unsafe {
        context.use_program(Some(shader.get_program()));
    }
    unsafe {
        context.clear_color(1.0, 1.0, 1.0, 1.0);
    }
    unsafe {
        context.clear(glow::COLOR_BUFFER_BIT);
    }
    unsafe {
        context.draw_arrays(glow::TRIANGLES, 0, 3);
    }
}
