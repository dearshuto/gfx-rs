use glow::HasContext;
use sjgfx_interface::{CommandBufferInfo, DeviceInfo, QueueInfo};

fn main() {
    sjgfx_glow::initialize();
    // let mut instance = sjgfx_glow::vi::Instance::new();
    // let id = instance.create_display();
    let mut device = sjgfx_glow::DeviceGlow::new(&DeviceInfo::new());
    let mut command_buffer = sjgfx_glow::CommandBufferGlow::new(&device, &CommandBufferInfo::new());
    let mut queue = sjgfx_glow::QueueGlow::new(&device, &QueueInfo::new());

    let vertex_shader_source = include_str!("resources/hello_triangle.vs");
    let pixel_shader_source = include_str!("resources/hello_triangle.fs");

    let shader = sjgfx_glow::ShaderGlow::new(
        &mut device,
        &sjgfx_interface::ShaderInfo::new()
            .set_vertex_shader_source(vertex_shader_source)
            .set_pixel_shader_source(pixel_shader_source),
    );

    command_buffer.begin();
    command_buffer.set_shader(&shader);
    command_buffer.end();
    queue.execute(&command_buffer);

    let error = unsafe { device.clone_context().get_error() };
    if error != glow::NO_ERROR {
        println!("ERROR");
    }

    sjgfx_glow::finalize();
}
