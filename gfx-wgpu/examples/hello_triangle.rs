use sjgfx_interface::{
    CommandBufferInfo, DeviceInfo, PrimitiveTopology, QueueInfo, ShaderInfo, SwapChainInfo,
};
use sjgfx_wgpu::{CommandBufferWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu, SwapChainWgpu};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let device = DeviceWgpu::new_as_graphics(&DeviceInfo::new(), &window);
    let mut swap_chain = SwapChainWgpu::new(&device, &SwapChainInfo::new());
    let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

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
    let shader = ShaderWgpu::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(&vertex_shader_binary.as_binary_u8())
            .set_pixel_shader_binary(&pixel_shader_binary.as_binary_u8()),
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
