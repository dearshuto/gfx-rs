use std::mem::size_of;

use glow::HasContext;
use sjgfx_glow::VertexStateGlow;
use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IBuffer, ICommandBuffer,
    IVertexState, PrimitiveTopology, QueueInfo, VertexAttributeStateInfo, VertexBufferStateInfo,
    VertexStateInfo,
};

fn main() {
    sjgfx_glow::initialize();
    let mut instance = sjgfx_glow::vi::Instance::new();
    let id = instance.create_display();
    let display = instance.try_get_display(id).unwrap();
    let mut device = sjgfx_glow::DeviceGlow::new_with_display(&DeviceInfo::new(), &display);
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

    let vertex_state = VertexStateGlow::new(
        &device,
        &VertexStateInfo::new()
            .set_attribute_state_info_array([VertexAttributeStateInfo::new()
                .set_buffer_index(0)
                .set_slot(0)
                .set_format(AttributeFormat::Float32_32)
                .set_offset(0)])
            .set_buffer_state_info_array([
                VertexBufferStateInfo::new().set_stride((size_of::<f32>() * 2) as i64)
            ]),
    );

    let vertex_buffer = sjgfx_glow::BufferGlow::new(
        &mut device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(256),
    );
    let error = unsafe { device.clone_context().get_error() };
    if error != glow::NO_ERROR {
        println!("ERROR: {}", error);
    }

    vertex_buffer.map_as_slice_mut(|data: &mut [f32]| {
        data[0] = -0.5;
        data[1] = -0.5;
        data[2] = 0.5;
        data[3] = -0.5;
        data[4] = 0.0;
        data[5] = 0.6;
    });
    let error = unsafe { device.clone_context().get_error() };
    if error != glow::NO_ERROR {
        println!("MAP ERROR: {}", error);
    }

    while instance.should_update() {
        command_buffer.begin();
        command_buffer.set_shader(&shader);
        command_buffer.set_vertex_state(&vertex_state);
        command_buffer.set_vertex_buffer(0, &vertex_buffer);
        command_buffer.draw(PrimitiveTopology::TriangleList, 3, 0);
        command_buffer.end();
        queue.execute(&command_buffer);
        instance
            .try_get_display(id)
            .unwrap()
            .window
            .swap_buffers()
            .unwrap();
    }

    sjgfx_glow::finalize();
}
