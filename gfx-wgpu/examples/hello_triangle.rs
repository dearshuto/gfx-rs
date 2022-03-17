use sjgfx_interface::{
    AttributeFormat, CommandBufferInfo, DeviceInfo, IColorTargetView, ICommandBuffer, IDevice,
    IQueue, IShader, ISwapChain, IVertexState, PrimitiveTopology, QueueInfo, ShaderInfo,
    SwapChainInfo, VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};
use sjgfx_wgpu::{
    BufferWgpu, ColorTargetViewWgpu, CommandBufferWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu,
    SwapChainWgpu, VertexStateWgpu,
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
    run::<
        DeviceWgpu,
        QueueWgpu,
        ShaderWgpu,
        CommandBufferWgpu,
        SwapChainWgpu,
        ColorTargetViewWgpu,
        VertexStateWgpu,
        BufferWgpu,
    >();
}

fn run<
    TDevice,
    TQueue,
    TShader,
    TCommandBuffer,
    TSwapChain,
    TColorTargetView,
    TVertexState,
    TBuffer,
>()
where
    TDevice: IDevice,
    TQueue: IQueue<
        DeviceType = TDevice,
        CommandBufferType = TCommandBuffer,
        SwapChainType = TSwapChain,
    >,
    TShader: IShader<DeviceType = TDevice>,
    TCommandBuffer: ICommandBuffer<
        DeviceType = TDevice,
        ShaderType = TShader,
        ColorTargetViewType = TColorTargetView,
        VertexStateType = TVertexState,
        BufferType = TBuffer,
    >,
    TSwapChain: ISwapChain<DeviceType = TDevice, ColorTargetViewType = TColorTargetView>,
    TColorTargetView: IColorTargetView,
    TVertexState: IVertexState<DeviceType = TDevice>,
{
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut device = TDevice::new_with_surface(&DeviceInfo::new(), &window, &event_loop);
    let mut swap_chain = TSwapChain::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );
    let mut queue = TQueue::new(&device, &QueueInfo::new());
    let mut command_buffer = TCommandBuffer::new(&device, &CommandBufferInfo::new());

    let mut compiler = shaderc::Compiler::new().unwrap();
    let vertex_shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_bufferless_triangle.vs"),
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
    let shader = TShader::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(&vertex_shader_binary.as_binary_u8())
            .set_pixel_shader_binary(&pixel_shader_binary.as_binary_u8()),
    );

    let attribute_state_info_array = [VertexAttributeStateInfo::new()
        .set_buffer_index(0)
        .set_format(AttributeFormat::Float32_32_32)
        .set_offset(0)
        .set_slot(0)];
    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<Vertex>() as i64)];
    let vertex_state = TVertexState::new(
        &device,
        &VertexStateInfo::new()
            .set_attribute_state_info_array(attribute_state_info_array.into_iter())
            .set_buffer_state_info_array(vertex_buffer_state_info_array),
    );

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

    // GPU コマンドが全て完了するのを待つ
    queue.sync();
}
