use std::mem::size_of;

use gfx_websys::{
    BufferWsys, CommandBufferWsys, DeviceWsys, QueueWsys, ShaderWsys, VertexStateWsys,
};
use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IBuffer, ICommandBuffer,
    IDevice, IQueue, IShader, IVertexState, PrimitiveTopology, QueueInfo, ShaderInfo,
    VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};

#[repr(C)]
struct VertexData {
    x: f32,
    y: f32,
}

fn main() {
    let mut instance = sjvi::web_sys::Instance::new();
    let id = instance.create_display();
    let display = instance.try_get_display(&id).unwrap();

    let mut device = DeviceWsys::new_with_surface(&DeviceInfo::new(), &display);
    let mut queue = QueueWsys::new(&mut device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWsys::new(&mut device, &CommandBufferInfo::new());
    let shader = ShaderWsys::new(
        &mut device,
        &ShaderInfo::new()
            .set_vertex_shader_source(include_str!("resources/mandelbrot.vs"))
            .set_pixel_shader_source(include_str!("resources/mandelbrot.fs")),
    );

    let buffer = BufferWsys::new(
        &mut device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(size_of::<VertexData>() * 6),
    );
    buffer.map_as_slice_mut(|x: &mut [VertexData]| {
        x[0].x = -1.0;
        x[0].y = 1.0;
        x[1].x = -1.0;
        x[1].y = -1.0;
        x[2].x = 1.0;
        x[2].y = -1.0;

        x[3].x = -1.0;
        x[3].y = 1.0;
        x[4].x = 1.0;
        x[4].y = -1.0;
        x[5].x = 1.0;
        x[5].y = 1.0;
    });

    let vertex_state = VertexStateWsys::new(
        &mut device,
        &VertexStateInfo::new()
            .set_attribute_state_info_array([VertexAttributeStateInfo::new()
                .set_buffer_index(0)
                .set_format(AttributeFormat::Float32_32)
                .set_offset(0)
                .set_slot(0)])
            .set_buffer_state_info_array([
                VertexBufferStateInfo::new().set_stride(size_of::<VertexData>() as i64)
            ]),
    );

    command_buffer.begin();
    command_buffer.set_shader(&shader);
    command_buffer.set_vertex_buffer(0, &buffer);
    command_buffer.set_vertex_state(&vertex_state);
    command_buffer.draw(
        PrimitiveTopology::TriangleList,
        6, /*vertex_count*/
        0, /*vertex_offset*/
    );
    command_buffer.end();

    queue.execute(&command_buffer);
    queue.sync();
}
