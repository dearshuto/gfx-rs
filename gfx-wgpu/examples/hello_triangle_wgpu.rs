use sjgfx_interface::{
    CommandBufferInfo, DeviceInfo, IColorTargetView, ICommandBuffer, IDevice, IQueue, IShader,
    ISwapChain, IVertexState, PrimitiveTopology, QueueInfo, ShaderInfo, SwapChainInfo,
    TextureArrayRange,
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

    let mut compiler = sjgfx_util::ShaderCompiler::new();
    let vertex_shader_binary = compiler.create_binary(
        &include_str!("../../resources/examples/shaders/hello_bufferless_triangle.vs"),
        sjgfx_util::ShaderStage::Vertex,
    );
    let pixel_shader_binary = compiler.create_binary(
        &include_str!("../../resources/examples/shaders/hello_triangle.fs"),
        sjgfx_util::ShaderStage::Pixel,
    );
    let shader = TShader::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(&vertex_shader_binary)
            .set_pixel_shader_binary(&pixel_shader_binary),
    );

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    // スキャンバッファの取得
                    let mut next_scan_buffer_view =
                        swap_chain.acquire_next_scan_buffer_view(None, None);
                    //let next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view(&mut semaphore, &mut display_fence);

                    // スキャンバッファの取得を同期 (GPU)
                    // queue.sync_semaphore(&semaphore);

                    // コマンドを作成
                    command_buffer.begin();
                    command_buffer.clear_color(
                        &mut next_scan_buffer_view,
                        0.0,
                        0.1,
                        0.2,
                        1.0,
                        TextureArrayRange::new(),
                    );
                    command_buffer.set_shader(&shader);
                    command_buffer.set_render_targets(&[&next_scan_buffer_view], None);
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
