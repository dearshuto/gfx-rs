use sjgfx::{
    BufferBuilder, CommandBufferBuilder, DeviceBuilder, QueueBuilder, ShaderBuilder,
    SwapChainBuilder, VertexStateBuilder,
};
use sjgfx_interface::{
    AttributeFormat, IBuffer, PrimitiveTopology, VertexAttributeStateInfo, VertexBufferStateInfo,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

#[repr(C)]
struct Vertex {
    #[allow(dead_code)]
    pub x: f32,

    #[allow(dead_code)]
    pub y: f32,
}

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut device = DeviceBuilder::new().build_with_surface(&window, &event_loop);
    let mut swap_chain = SwapChainBuilder::new().build(&mut device);
    let mut queue = QueueBuilder::new().build(&device);
    let mut command_buffer = CommandBufferBuilder::new().build(&device);
    let vertex_buffer = BufferBuilder::new()
        .with_size(64)
        .enable_vertex_buffer()
        .build(&device);
    IBuffer::map_as_slice_mut(&vertex_buffer, |x| {
        x[0] = Vertex { x: -0.5, y: -0.5 };
        x[1] = Vertex { x: 0.5, y: -0.5 };
        x[2] = Vertex { x: 0.0, y: 0.5 };
    });

    let mut compiler = shaderc::Compiler::new().unwrap();
    let vertex_shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_triangle.vs"),
            shaderc::ShaderKind::Vertex,
            "test.glsl",
            "main",
            None,
        )
        .unwrap();
    let pixel_shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_triangle.fs"),
            shaderc::ShaderKind::Fragment,
            "test.glsl",
            "main",
            None,
        )
        .unwrap();
    let shader = ShaderBuilder::new()
        .set_vertex_shader_binary(vertex_shader_binary.as_binary_u8())
        .set_pixel_shader_binary(pixel_shader_binary.as_binary_u8())
        .build(&device);

    let vertex_attribute_state_info_array = [VertexAttributeStateInfo::new()
        .set_buffer_index(0)
        .set_format(AttributeFormat::Float32_32)
        .set_offset(0)
        .set_slot(0)];
    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<Vertex>() as i64)];
    let vertex_state = VertexStateBuilder::new()
        .set_vertex_attribute_states(vertex_attribute_state_info_array.into_iter())
        .set_vertex_buffer_states(vertex_buffer_state_info_array.into_iter())
        .build(&device);

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    // スキャンバッファの取得
                    let next_scan_buffer_view =
                        swap_chain.acquire_next_scan_buffer_view(None, None);
                    //let next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view(&mut semaphore, &mut display_fence);

                    // スキャンバッファの取得を同期 (GPU)
                    // queue.sync_semaphore(&semaphore);

                    // コマンドを作成
                    command_buffer.begin();
                    command_buffer.set_shader(&shader);
                    command_buffer.set_render_targets([next_scan_buffer_view].into_iter(), None);
                    command_buffer.set_vertex_state(&vertex_state);
                    command_buffer.set_vertex_buffer(0, &vertex_buffer);
                    command_buffer.draw(PrimitiveTopology::TriangleList, 3, 0);
                    command_buffer.end();

                    // コマンドをキューに積む
                    queue.execute(&command_buffer);
                    //queue.execute(&command_buffer, &fence);

                    // 結果の表示
                    queue.present(&mut swap_chain);

                    // コマンドを実行
                    queue.flush();

                    // スキャンバッファの取得を同期 (CPU)
                    // display_fence.sync();

                    // 前フレームのコマンドの実行を同期
                    // fence.sync();
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
