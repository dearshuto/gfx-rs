use std::mem::size_of;

use sjgfx_glow::{DeviceGlow, SwapChainGlow, VertexStateGlow};
use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IBuffer, ICommandBuffer,
    IQueue, ISwapChain, IVertexState, PrimitiveTopology, QueueInfo, SwapChainInfo,
    VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};

#[repr(C)]
struct VertexData {
    x: f32,
    y: f32,
    red: f32,
    green: f32,
    blue: f32,
}

fn main() {
    let mut instance = sjvi::glutin::Instance::new();
    let id = instance.create_display();
    let display = instance.try_get_display(id).unwrap();

    let mut device = DeviceGlow::new_with_display(&DeviceInfo::new(), &display);
    let mut swap_chain = SwapChainGlow::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );

    let mut command_buffer = sjgfx_glow::CommandBufferGlow::new(&device, &CommandBufferInfo::new());
    let mut queue = sjgfx_glow::QueueGlow::new(&mut device, &QueueInfo::new());

    let vertex_shader_source = include_str!("resources/shaders/draw_uv.vs");
    let pixel_shader_source = include_str!("resources/shaders/draw_uv.fs");
    let shader = sjgfx_glow::ShaderGlow::new(
        &mut device,
        &sjgfx_interface::ShaderInfo::new()
            .set_vertex_shader_source(vertex_shader_source)
            .set_pixel_shader_source(pixel_shader_source),
    );

    let vertex_state = VertexStateGlow::new(
        &device,
        &VertexStateInfo::new()
            .set_attribute_state_info_array([
                VertexAttributeStateInfo::new()
                    .set_buffer_index(0)
                    .set_slot(0)
                    .set_format(AttributeFormat::Float32_32)
                    .set_offset(0),
                VertexAttributeStateInfo::new()
                    .set_buffer_index(0)
                    .set_slot(1)
                    .set_format(AttributeFormat::Float32_32_32)
                    .set_offset(size_of::<f32>() as i64 * 2),
            ])
            .set_buffer_state_info_array([
                VertexBufferStateInfo::new().set_stride(size_of::<VertexData>() as i64)
            ]),
    );

    let vertex_buffer = sjgfx_glow::BufferGlow::new(
        &mut device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(size_of::<VertexData>() * 3),
    );

    vertex_buffer.map_as_slice_mut(|data: &mut [VertexData]| {
        data[0].x = -0.5;
        data[0].y = -0.5;
        data[0].red = 1.0;
        data[0].green = 0.0;
        data[0].blue = 0.0;

        data[1].x = 0.5;
        data[1].y = -0.5;
        data[1].red = 0.0;
        data[1].green = 1.0;
        data[1].blue = 0.0;

        data[2].x = 0.0;
        data[2].y = 0.5;
        data[2].red = 0.0;
        data[2].green = 0.0;
        data[2].blue = 1.0;
    });

    while instance.try_update() {
        let scan_buffer = swap_chain.acquire_next_scan_buffer_view(None, None);

        command_buffer.begin();
        command_buffer.set_render_targets(&[&scan_buffer], None);
        command_buffer.set_shader(&shader);
        command_buffer.set_vertex_state(&vertex_state);
        command_buffer.set_vertex_buffer(0, &vertex_buffer);
        command_buffer.draw(PrimitiveTopology::TriangleList, 3, 0);
        command_buffer.end();

        queue.execute(&command_buffer);
        queue.present(&mut swap_chain);
        queue.flush();
    }
}
