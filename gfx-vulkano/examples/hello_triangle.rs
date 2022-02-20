use sjgfx_interface::{
    BufferInfo, CommandBufferInfo, DeviceInfo, FenceInfo, PrimitiveTopology, QueueInfo, ShaderInfo,
    SwapChainInfo, VertexStateInfo,
};
use sjgfx_vulkano::{
    BufferVk, CommandBufferVk, DeviceVk, FenceVk, Float32_32, QueueVk, ShaderVk, SwapChainVk,
    VertexStateVk,
};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
};

fn main() {
    let mut event_loop = EventLoop::new();
    let device = DeviceVk::new_as_graphics(&DeviceInfo::new(), &event_loop);
    let mut swap_chain = SwapChainVk::new(&device, &SwapChainInfo::new());
    let mut command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());
    let mut queue = QueueVk::new(&device, &QueueInfo::new());
    let mut fence = FenceVk::new(&device, &FenceInfo::new());

    // シェーダ
    let vertex_shader_source = include_str!("../../resources/examples/shaders/hello_triangle.vs");
    let pixel_shader_source = include_str!("../../resources/examples/shaders/hello_triangle.fs");
    let mut compiler = shaderc::Compiler::new().unwrap();
    let vertex_shader_binary = compiler
        .compile_into_spirv(
            &vertex_shader_source,
            shaderc::ShaderKind::Vertex,
            "test.glsl",
            "main",
            None,
        )
        .unwrap();
    let pixel_shader_binary = compiler
        .compile_into_spirv(
            &pixel_shader_source,
            shaderc::ShaderKind::Fragment,
            "test.fs",
            "main",
            None,
        )
        .unwrap();
    let shader = ShaderVk::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(vertex_shader_binary.as_binary_u8())
            .set_pixel_shader_binary(pixel_shader_binary.as_binary_u8()),
    );

    // 頂点バッファ
    let vertex_buffer = BufferVk::new_as_array::<Float32_32>(&device, &BufferInfo::new());

    // 頂点ステート
    let vertex_state = {
        let attribute_state_infos = Vec::new(); // TODO
        let buffer_state_infos = Vec::new(); // TODO
        VertexStateVk::new(
            &device,
            &VertexStateInfo::new()
                .set_attribute_state_info_array(attribute_state_infos)
                .set_buffer_state_info_array(buffer_state_infos),
        )
    };

    loop {
        event_loop.run_return(|_event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            let next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view(&mut fence);

            {
                command_buffer.begin();
                command_buffer.set_render_targets([next_scan_buffer_view].into_iter(), None);
                command_buffer.set_shader(&shader);
                command_buffer.set_vertex_state(&vertex_state);
                command_buffer.set_vertex_buffer(0, &vertex_buffer);
                command_buffer.draw(
                    PrimitiveTopology::TriangleList,
                    0, /*count*/
                    0, /*offset*/
                );
                command_buffer.end();
            }

            queue.execute(&command_buffer);
            queue.present(&mut swap_chain);
            queue.flush();
            queue.sync();
        });
    }
}
