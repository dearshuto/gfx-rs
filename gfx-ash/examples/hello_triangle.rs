use sjgfx_ash::{
    CommandBufferAsh, DeviceAsh, FenceAsh, QueueAsh, SemaphoreAsh, ShaderAsh, SwapChainAsh,
};
use sjgfx_interface::{
    CommandBufferInfo, DeviceInfo, FenceInfo, ICommandBuffer, IDevice, IFence, IQueue, ISemaphore,
    IShader, ISwapChain, PrimitiveTopology, QueueInfo, SemaphoreInfo, ShaderInfo, SwapChainInfo,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

fn main() {
    run::<DeviceAsh, QueueAsh, CommandBufferAsh, ShaderAsh, SwapChainAsh, SemaphoreAsh, FenceAsh>();
}

fn run<TDevice, TQueue, TCommandBuffer, TShader, TSwapChain, TSemaphore, TFence>()
where
    TDevice: IDevice,
    TQueue: IQueue<
        DeviceType = TDevice,
        CommandBufferType = TCommandBuffer,
        SwapChainType = TSwapChain,
    >,
    TCommandBuffer: ICommandBuffer<
        DeviceType = TDevice,
        ShaderType = TShader,
        ColorTargetViewType = TSwapChain::ColorTargetViewType,
    >,
    TShader: IShader<DeviceType = TDevice>,
    TSwapChain: ISwapChain<DeviceType = TDevice, SemaphoreType = TSemaphore>,
    TSemaphore: ISemaphore<DeviceType = TDevice>,
    TFence: IFence<DeviceType = TDevice>,
{
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let device = TDevice::new_with_surface(&DeviceInfo::new(), &window, &event_loop);
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

    let mut swap_chain = TSwapChain::new(&device, &SwapChainInfo::new());

    let mut semaphore = TSemaphore::new(&device, &SemaphoreInfo::new());
    let mut _fence = TFence::new(&device, &FenceInfo::new());

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    let next_scan_buffer_view =
                        swap_chain.acquire_next_scan_buffer_view(Some(&mut semaphore), None);

                    command_buffer.begin();
                    command_buffer.set_render_targets([next_scan_buffer_view].into_iter(), None);
                    command_buffer.set_shader(&shader);
                    command_buffer.draw(
                        PrimitiveTopology::TriangleList,
                        3, /*vertex_count*/
                        0, /*vertex_offset*/
                    );
                    command_buffer.end();

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
                    *control_flow = ControlFlow::Exit
                }
                _ => {}
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
