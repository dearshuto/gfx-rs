use sjgfx_interface::{
    CommandBufferInfo, DeviceInfo, IColorTargetView, ICommandBuffer, IDevice, IQueue, IShader,
    ISwapChain, IVertexState, PrimitiveTopology, QueueInfo, ShaderInfo, SwapChainInfo,
    TextureArrayRange,
};
use sjgfx_wgpu::{
    BufferWgpu, ColorTargetViewWgpu, CommandBufferWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu,
    SwapChainWgpu, VertexStateWgpu,
};
use sjvi::IDisplay;
use sjvi::IInstance;

fn main() {
    run::<
        sjvi::winit::Instance,
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
    TInstance,
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
    TInstance: IInstance<Display = TDevice::Display, DisplayId = sjvi::winit::Id>,
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
    let mut instance = TInstance::new();
    let id = instance.create_display();

    let mut device = {
        let displaty = instance.try_get_display(&id).unwrap();
        TDevice::new_with_surface(&DeviceInfo::new(), displaty)
    };
    let mut swap_chain = TSwapChain::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );
    let mut queue = TQueue::new(&mut device, &QueueInfo::new());
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
        &mut device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(&vertex_shader_binary)
            .set_pixel_shader_binary(&pixel_shader_binary),
    );

    while instance.try_update() {
        let display = instance.try_get_display(&id).unwrap();
        if display.is_redraw_requested() {
            // スキャンバッファの取得
            let mut next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view(None, None);
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
        }

        display.listen(&mut swap_chain);
    }

    // GPU コマンドが全て完了するのを待つ
    queue.sync();
}
