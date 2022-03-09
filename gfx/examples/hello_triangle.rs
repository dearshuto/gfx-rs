use sjgfx::wgpu::*;
use sjgfx::{
    CommandBufferBuilder, DeviceBuilder, QueueBuilder, SemaphoreBuilder, ShaderBuilder,
    SwapChainBuilder,
};
use sjgfx_interface::PrimitiveTopology;
//use sjgfx::vulkano::*;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::{
    event_loop::EventLoop, platform::run_return::EventLoopExtRunReturn, window::WindowBuilder,
};

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let device = DeviceBuilder::new().build_widh_surface(&window, &event_loop);
    let mut queue = QueueBuilder::new().build(&device);
    let mut command_buffer = CommandBufferBuilder::new().build(&device);

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
    let shader = ShaderBuilder::new()
        .set_vertex_shader_binary(vertex_shader_binary.as_binary_u8())
        .set_pixel_shader_binary(pixel_shader_binary.as_binary_u8())
        .build(&device);

    let mut swap_chain = SwapChainBuilder::new().build(&device);
    let mut semaphore = SemaphoreBuilder::new().build(&device);

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
    }
}
