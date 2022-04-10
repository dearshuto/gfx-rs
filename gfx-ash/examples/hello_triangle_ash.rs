use sjgfx_ash::{
    CommandBufferAsh, DeviceAsh, FenceAsh, QueueAsh, SemaphoreAsh, ShaderAsh, SwapChainAsh,
};
use sjgfx_interface::{
    CommandBufferInfo, DeviceInfo, PrimitiveTopology, QueueInfo, ShaderInfo, SwapChainInfo,
    TextureArrayRange,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

fn main() {
    run();
}

fn run() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(PhysicalSize::new(1280, 960))
        .build(&event_loop)
        .unwrap();

    let mut device = DeviceAsh::new_with_surface(&DeviceInfo::new(), &window);
    let mut queue = QueueAsh::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferAsh::new(&device, &CommandBufferInfo::new());

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
    let shader = ShaderAsh::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(&vertex_shader_binary.as_binary_u8())
            .set_pixel_shader_binary(&pixel_shader_binary.as_binary_u8()),
    );

    let mut swap_chain = SwapChainAsh::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );

    let mut semaphore = SemaphoreAsh::new(&device);
    let mut _fence = FenceAsh::new(&device);

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    let mut next_scan_buffer_view =
                        swap_chain.acquire_next_scan_buffer_view(Some(&mut semaphore), None);

                    command_buffer.begin();
                    command_buffer.clear_color(
                        &mut next_scan_buffer_view,
                        0.4,
                        0.1,
                        0.1,
                        0.0,
                        TextureArrayRange::new(),
                    );
                    command_buffer.set_render_targets(&[&next_scan_buffer_view], None);
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
