use sjgfx_interface::{
    CommandBufferInfo, DeviceInfo, IColorTargetView, ICommandBuffer, IDevice, IQueue, IShader,
    ISwapChain, IVertexState, PrimitiveTopology, QueueInfo, ShaderInfo, SwapChainInfo,
    TextureArrayRange,
};
use sjgfx_wgpu::{
    BufferWgpu, ColorTargetViewWgpu, CommandBufferWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu,
    SwapChainWgpu, VertexStateWgpu,
};
use web_sys::window;
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        let query_string = web_sys::window().unwrap().location().search().unwrap();
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
    }

    let mut device = DeviceWgpu::new_as_graphics(&DeviceInfo::new(), &window);
    // let mut swap_chain = SwapChainWgpu::new(
    //     &mut device,
    //     &SwapChainInfo::new().with_width(1280).with_height(960),
    // );
    // let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    // let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    // let mut compiler = sjgfx_util::ShaderCompiler::new();
    // let vertex_shader_binary = compiler.create_binary(
    //     &include_str!("../../resources/examples/shaders/hello_bufferless_triangle.vs"),
    //     sjgfx_util::ShaderStage::Vertex,
    // );
    // let pixel_shader_binary = compiler.create_binary(
    //     &include_str!("../../resources/examples/shaders/hello_triangle.fs"),
    //     sjgfx_util::ShaderStage::Pixel,
    // );
    // let shader = ShaderWgpu::new(
    //     &device,
    //     &ShaderInfo::new()
    //         .set_vertex_shader_binary(&vertex_shader_binary)
    //         .set_pixel_shader_binary(&pixel_shader_binary),
    // );

    // event_loop.run(move |event, _, control_flow| {
    //     // スキャンバッファの取得
    //     let mut next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view(None, None);
    //     //let next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view(&mut semaphore, &mut display_fence);

    //     // スキャンバッファの取得を同期 (GPU)
    //     // queue.sync_semaphore(&semaphore);

    //     // コマンドを作成
    //     command_buffer.begin();
    //     command_buffer.clear_color(
    //         &mut next_scan_buffer_view,
    //         0.0,
    //         0.1,
    //         0.2,
    //         1.0,
    //         TextureArrayRange::new(),
    //     );
    //     command_buffer.set_shader(&shader);
    //     command_buffer.set_render_targets(&[&next_scan_buffer_view], None);
    //     command_buffer.draw(PrimitiveTopology::TriangleList, 3, 0);
    //     command_buffer.end();

    //     // コマンドをキューに積む
    //     queue.execute(&command_buffer);
    //     //queue.execute(&command_buffer, &fence);

    //     // 結果の表示
    //     queue.present(&mut swap_chain);

    //     // コマンドを実行
    //     queue.flush();
    // });
}
