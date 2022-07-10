use glow::HasContext;
use sjgfx_interface::{CommandBufferInfo, DeviceInfo, QueueInfo, BufferInfo, GpuAccess, IBuffer};

fn main() {
    sjgfx_glow::initialize();
    let mut instance = sjgfx_glow::vi::Instance::new();
    let id = instance.create_display();
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

    let vertex_buffer = sjgfx_glow::BufferGlow::new(&mut device, &BufferInfo::new().set_gpu_access_flags(GpuAccess::VERTEX_BUFFER).set_size(256));
    vertex_buffer.map_as_slice_mut(|data: &mut [f32]| {
        data[0] = 0.0;
        data[1] = 0.1;
    });
    let error = unsafe { device.clone_context().get_error() };
    if error != glow::NO_ERROR {
        println!("ERROR: {}", error);
    }

    while instance.should_update() {
        command_buffer.begin();
        command_buffer.set_shader(&shader);
        command_buffer.end();
        queue.execute(&command_buffer);
        instance.try_get_display(id).unwrap().window.swap_buffers().unwrap();
    }

    sjgfx_glow::finalize();
}
