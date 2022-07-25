use std::mem::size_of;

use glow::HasContext;
use sjgfx_glow::{
    BufferGlow, CommandBufferGlow, DeviceGlow, QueueGlow, ShaderGlow, SwapChainGlow,
    VertexStateGlow,
};
use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IBuffer, ICommandBuffer,
    IQueue, ISwapChain, IVertexState, PrimitiveTopology, QueueInfo, ShaderInfo, SwapChainInfo,
    VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};

#[cfg(any(not(target_arch = "wasm32")))]
use sjvi::glutin::Instance;

#[cfg(target_arch = "wasm32")]
use sjvi::web_sys::Instance;

#[cfg(target_arch = "wasm32")]
use sjvi::IInstance;

fn main() {
    let mut instance = Instance::new();
    let id = instance.create_display();

    #[cfg(any(not(target_arch = "wasm32")))]
    let display = instance.try_get_display(id).unwrap();

    #[cfg(target_arch = "wasm32")]
    let display = instance.try_get_display(&id).unwrap();

    let mut device = DeviceGlow::new_with_display(&DeviceInfo::new(), &display);
    let buffer = BufferGlow::new(
        &mut device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(64),
    );
    buffer.map_as_slice_mut(|_x: &mut [f32]| {});

    let mut queue = QueueGlow::new(&mut device, &QueueInfo::new());
    let mut swap_chain = SwapChainGlow::new(&mut device, &SwapChainInfo::new());
    let mut command_buffer = CommandBufferGlow::new(&mut device, &CommandBufferInfo::new());

    let shader = ShaderGlow::new(
        &mut device,
        &ShaderInfo::new()
            .set_vertex_shader_source(include_str!("resources/shaders/hello_triangle.vs"))
            .set_pixel_shader_source(include_str!("resources/shaders/hello_triangle.fs")),
    );
    let vertex_state = VertexStateGlow::new(
        &mut device,
        &VertexStateInfo::new()
            .set_attribute_state_info_array([VertexAttributeStateInfo::new()
                .set_buffer_index(0)
                .set_format(AttributeFormat::Float32_32)
                .set_offset(0)
                .set_slot(0)])
            .set_buffer_state_info_array([
                VertexBufferStateInfo::new().set_stride(size_of::<f32>() as i64 * 2)
            ]),
    );
    let vertex_buffer = BufferGlow::new(
        &mut device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(size_of::<f32>() * 6),
    );
    vertex_buffer.map_as_slice_mut(|x: &mut [f32]| {
        x[0] = -0.5;
        x[1] = -0.5;
        x[2] = 0.5;
        x[3] = -0.5;
        x[4] = 0.0;
        x[5] = 0.5;
    });

    let gl = device.clone_context();

    let vao = unsafe{ gl.create_vertex_array() }.unwrap();
    unsafe{ gl.bind_vertex_array(Some(vao)) }
    unsafe{ gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer.get_handle())) }
    unsafe{ gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, size_of::<f32>() as i32 * 2, 0) };
    unsafe{ gl.bind_buffer(glow::ARRAY_BUFFER, None) }
    unsafe{ gl.bind_vertex_array(None) }

    while {
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            gl.bind_framebuffer(glow::FRAMEBUFFER, None);

            gl.use_program(Some(shader.get_program()));
            gl.bind_vertex_array(Some(vao));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }


        // let _next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view(None, None);

        // command_buffer.begin();
        // command_buffer.set_render_targets(&[&next_scan_buffer_view], None);
        // command_buffer.set_shader(&shader);
        // command_buffer.set_vertex_state(&vertex_state);
        // command_buffer.set_vertex_buffer(0, &vertex_buffer);
        // command_buffer.draw(
        //     PrimitiveTopology::TriangleList,
        //     3, /*vertex_count*/
        //     0, /*offset*/
        // );
        // command_buffer.end();

        // queue.execute(&command_buffer);
        // queue.present(&mut swap_chain); // wasm だといらない？
        queue.flush();

        instance.try_update()
    } {}
}
