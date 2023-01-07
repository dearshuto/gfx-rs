use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, FenceInfo, GpuAccess,
    PrimitiveTopology, QueueInfo, ScissorStateInfo, ShaderInfo, SwapChainInfo,
    VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo, ViewportScissorStateInfo,
    ViewportStateInfo,
};
use sjgfx_vulkano::{
    BufferVk, CommandBufferVk, DeviceVk, FenceVk, QueueVk, ShaderVk, SwapChainVk, VertexStateVk,
    ViewportScissorStateVk,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let device = DeviceVk::new_from_handle(&DeviceInfo::new(), &window);
    let mut swap_chain = SwapChainVk::new(&device, &SwapChainInfo::new());
    let mut command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());
    let mut queue = QueueVk::new(&device, &QueueInfo::new());
    let mut fence = FenceVk::new(&device, &FenceInfo::new());

    let viewport_scissor_state = ViewportScissorStateVk::new(
        &device,
        &ViewportScissorStateInfo::new()
            .set_viewport_state_info_array(&[ViewportStateInfo::new()
                .set_width(1280.0)
                .set_height(960.0)])
            .set_scissor_state_info_array(&[ScissorStateInfo::new()
                .set_width(1280)
                .set_height(960)]),
    );

    // シェーダ
    let mut compiler = sjgfx_util::ShaderCompiler::new();
    let vertex_shader_source = include_str!("../../resources/examples/shaders/hello_triangle.vs");
    let pixel_shader_source = include_str!("../../resources/examples/shaders/hello_triangle.fs");
    let vertex_shader_binary =
        compiler.create_binary(vertex_shader_source, sjgfx_util::ShaderStage::Vertex);
    let pixel_shader_binary =
        compiler.create_binary(pixel_shader_source, sjgfx_util::ShaderStage::Pixel);
    let shader = ShaderVk::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(&vertex_shader_binary)
            .set_pixel_shader_binary(&pixel_shader_binary),
    );

    // 頂点バッファ
    let vertex_buffer = BufferVk::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(std::mem::size_of::<f32>() * 2 * 3),
    );
    vertex_buffer.map_as_array_mut(|x: &mut [f32]| {
        x[0] = 0.0;
        x[1] = 0.0;

        x[2] = -0.5;
        x[3] = -0.5;

        x[4] = 0.5;
        x[5] = -0.5;
    });

    // 頂点ステート
    let vertex_state = {
        let attribute_state_infos = [VertexAttributeStateInfo::new()
            .set_format(AttributeFormat::Float32_32)
            .set_buffer_index(0)
            .set_offset(0)
            .set_slot(0)];
        let buffer_state_infos =
            [VertexBufferStateInfo::new().set_stride((std::mem::size_of::<f32>() * 2) as i64)];
        VertexStateVk::new(
            &device,
            &VertexStateInfo::new()
                .set_attribute_state_info_array(attribute_state_infos)
                .set_buffer_state_info_array(buffer_state_infos),
        )
    };

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    queue.sync();

                    let next_scan_buffer_view =
                        swap_chain.acquire_next_scan_buffer_view(None, Some(&mut fence));

                    {
                        command_buffer.begin();
                        command_buffer.set_render_targets(&[&next_scan_buffer_view], None);
                        command_buffer.set_shader(&shader);
                        command_buffer.set_viewport_scissor_state(&viewport_scissor_state);
                        command_buffer.set_vertex_state(&vertex_state);
                        command_buffer.set_vertex_buffer(0, &vertex_buffer);
                        command_buffer.draw(
                            PrimitiveTopology::TriangleList,
                            3, /*count*/
                            0, /*offset*/
                        );
                        command_buffer.end();
                    }

                    queue.execute(&command_buffer);
                    queue.present(&mut swap_chain);
                    queue.flush();
                    queue.sync();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    should_close = true;
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
        });
    }
}
