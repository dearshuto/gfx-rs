use sjgfx::api::IApi;

use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IBuffer,
    ICommandBuffer, IDevice, IQueue, ISemaphore, IShader, ISwapChain,
    IVertexState, PrimitiveTopology, QueueInfo, SemaphoreInfo, ShaderInfo, SwapChainInfo,
    VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};

use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;
use winit::{event::Event, event_loop::ControlFlow};

fn main() {
    if cfg!(feature = "backend-wgpu") {
        run::<sjgfx::api::Wgpu>();
    }
    else if cfg!(feature = "backend-ash") {
        run::<sjgfx::api::Ash>();
    }
    else {
        println!("help: cargon run --release --bin mandelbrot --features backend-<ash/wgpu>")
    }
}

fn run<TApi: IApi>()
{
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut device = TApi::Device::new_with_surface(&DeviceInfo::new(), &window, &event_loop);
    let mut queue = TApi::Queue::new(&device, &QueueInfo::new());
    let mut command_buffer = TApi::CommandBuffer::new(&device, &CommandBufferInfo::new());
    let mut swap_chain = TApi::SwapChain::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );

    let vertex_shader_binary = include_bytes!("../outputs/resources/shaders/mandelbrot.vs.spv");
    let pixel_shader_binary = include_bytes!("../outputs/resources/shaders/mandelbrot.fs.spv");
    let shader = TApi::Shader::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(vertex_shader_binary)
            .set_pixel_shader_binary(pixel_shader_binary),
    );

    let vertex_attribute_state_info_array = [VertexAttributeStateInfo::new()
        .set_slot(0)
        .set_format(AttributeFormat::Float32_32)
        .set_offset(0)
        .set_buffer_index(0)];
    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride((std::mem::size_of::<f32>() * 2) as i64)];
    let vertex_state_info = VertexStateInfo::new()
        .set_attribute_state_info_array(vertex_attribute_state_info_array.into_iter())
        .set_buffer_state_info_array(vertex_buffer_state_info_array.into_iter());
    let vertex_state = TApi::VertexState::new(&device, &vertex_state_info);

    let buffer_info = BufferInfo::new()
        .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
        .set_size(128);

    // 画面いっぱいに四角形を描く
    let vertex_buffer = TApi::Buffer::new(&device, &buffer_info);
    vertex_buffer.map_as_slice_mut(|mapped_data: &mut [f32]| {
        mapped_data[0] = -1.0;
        mapped_data[1] = 1.0;
        mapped_data[2] = -1.0;
        mapped_data[3] = -1.0;
        mapped_data[4] = 1.0;
        mapped_data[5] = -1.0;

        mapped_data[6] = -1.0;
        mapped_data[7] = 1.0;
        mapped_data[8] = 1.0;
        mapped_data[9] = -1.0;
        mapped_data[10] = 1.0;
        mapped_data[11] = 1.0;
    });
    vertex_buffer.flush_mapped_range(0, 128);

    let mut semaphore = TApi::Semaphore::new(&device, &SemaphoreInfo::new());

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    // queue.sync_semaphore(&mut semaphore);

                    let next_scan_buffer_view =
                        swap_chain.acquire_next_scan_buffer_view(Some(&mut semaphore), None);

                    command_buffer.begin();
                    command_buffer.set_render_targets([next_scan_buffer_view].into_iter(), None);
                    command_buffer.set_shader(&shader);
                    command_buffer.set_vertex_state(&vertex_state);
                    command_buffer.set_vertex_buffer(0, &vertex_buffer);
                    command_buffer.draw(
                        PrimitiveTopology::TriangleList,
                        6, /*vertex_count*/
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
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
        });
        std::thread::sleep(std::time::Duration::from_millis(32));
    }
}
