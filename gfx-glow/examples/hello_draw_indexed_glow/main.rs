use std::mem::size_of;

use sjgfx_glow::{BufferGlow, VertexStateGlow};
use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IBuffer, ICommandBuffer,
    IQueue, ISwapChain, IVertexState, IndexFormat, PrimitiveTopology, QueueInfo, ShaderInfo,
    SwapChainInfo, VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};

#[repr(C)]
struct VertexData {
    x: f32,
    y: f32,
}

fn main() {
    let mut instance = sjvi::glutin::Instance::new();
    let id = instance.create_display();
    let display = instance.try_get_display(id).unwrap();

    let mut device = sjgfx_glow::DeviceGlow::new_with_display(&DeviceInfo::new(), &display);
    let mut queue = sjgfx_glow::QueueGlow::new(&mut device, &QueueInfo::new());
    let mut swap_chain = sjgfx_glow::SwapChainGlow::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );
    let mut command_buffer = sjgfx_glow::CommandBufferGlow::new(&device, &CommandBufferInfo::new());
    let shader = sjgfx_glow::ShaderGlow::new(
        &mut device,
        &ShaderInfo::new()
            .set_vertex_shader_source(include_str!("resources/shaders/draw_indexed.vs"))
            .set_pixel_shader_source(include_str!("resources/shaders/draw_indexed.fs")),
    );

    let vertex_state = VertexStateGlow::new(
        &device,
        &VertexStateInfo::new()
            .set_attribute_state_info_array([VertexAttributeStateInfo::new()
                .set_buffer_index(0)
                .set_format(AttributeFormat::Float32_32)
                .set_offset(0)
                .set_slot(0)])
            .set_buffer_state_info_array([
                VertexBufferStateInfo::new().set_stride((size_of::<f32>() * 2) as i64)
            ]),
    );
    let vertex_buffer = BufferGlow::new(
        &mut device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(size_of::<f32>() * 2 * 4),
    );
    vertex_buffer.map_as_slice_mut(|x: &mut [VertexData]| {
        x[0].x = -0.5;
        x[0].y = 0.5;
        x[1].x = -0.5;
        x[1].y = -0.5;
        x[2].x = 0.5;
        x[2].y = -0.5;
        x[3].x = 0.5;
        x[3].y = 0.5;
    });

    let index_buffer = BufferGlow::new(
        &mut device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::INDEX_BUFFER)
            .set_size(size_of::<u32>() * 6),
    );
    index_buffer.map_as_slice_mut(|x: &mut [u32]| {
        x[0] = 0;
        x[1] = 1;
        x[2] = 2;
        x[3] = 0;
        x[4] = 2;
        x[5] = 3;
    });

    while instance.try_update() {
        let display = instance.try_get_display(id).unwrap();
        if display.is_redraw_requested() {
            let scan_buffer = swap_chain.acquire_next_scan_buffer_view(None, None);

            command_buffer.begin();
            command_buffer.set_render_targets(&[&scan_buffer], None);
            command_buffer.set_shader(&shader);
            command_buffer.set_vertex_state(&vertex_state);
            command_buffer.set_vertex_buffer(0, &vertex_buffer);
            command_buffer.draw_indexed(
                PrimitiveTopology::TriangleList,
                IndexFormat::Uint32,
                &index_buffer,
                6, /*index_count*/
                0, /*base_vertex*/
            );
            command_buffer.end();

            queue.execute(&command_buffer);
            queue.present(&mut swap_chain);
            queue.flush();
        }
    }
}
