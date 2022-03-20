#[cfg(feature = "backend-ash")]
use sjgfx_ash::{
    BufferAsh, ColorTargetViewAsh, CommandBufferAsh, DeviceAsh, QueueAsh, SemaphoreAsh, ShaderAsh,
    SwapChainAsh, VertexStateAsh,
};

use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IBuffer,
    IColorTargetView, ICommandBuffer, IDevice, IQueue, ISemaphore, IShader, ISwapChain,
    IVertexState, PrimitiveTopology, QueueInfo, SemaphoreInfo, ShaderInfo, SwapChainInfo,
    VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};

#[cfg(feature = "backend-wgpu")]
use sjgfx_wgpu::{
    BufferWgpu, ColorTargetViewWgpu, CommandBufferWgpu, DeviceWgpu, QueueWgpu, SemaphoreWgpu,
    ShaderWgpu, SwapChainWgpu, VertexStateWgpu,
};

use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;
use winit::{event::Event, event_loop::ControlFlow};

fn main() {
    #[cfg(feature = "backend-ash")]
    run::<
        DeviceAsh,
        QueueAsh,
        CommandBufferAsh,
        SwapChainAsh,
        ColorTargetViewAsh,
        ShaderAsh,
        VertexStateAsh,
        BufferAsh,
        SemaphoreAsh,
    >();

    #[cfg(feature = "backend-wgpu")]
    run::<
        DeviceWgpu,
        QueueWgpu,
        CommandBufferWgpu,
        SwapChainWgpu,
        ColorTargetViewWgpu,
        ShaderWgpu,
        VertexStateWgpu,
        BufferWgpu,
        SemaphoreWgpu,
    >();
}

fn run<
    TDevice,
    TQueue,
    TCommandBuffer,
    TSwapChain,
    TColorTargetView,
    TShader,
    TVertexState,
    TBuffer,
    TSemaphore,
>()
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
        BufferType = TBuffer,
        VertexStateType = TVertexState,
        ColorTargetViewType = TColorTargetView,
    >,
    TSwapChain: ISwapChain<
        DeviceType = TDevice,
        ColorTargetViewType = TColorTargetView,
        SemaphoreType = TSemaphore,
    >,
    TColorTargetView: IColorTargetView<DeviceType = TDevice>,
    TShader: IShader<DeviceType = TDevice>,
    TVertexState: IVertexState<DeviceType = TDevice>,
    TBuffer: IBuffer<DeviceType = TDevice>,
    TSemaphore: ISemaphore<DeviceType = TDevice>,
{
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut device = TDevice::new_with_surface(&DeviceInfo::new(), &window, &event_loop);
    let mut queue = TQueue::new(&device, &QueueInfo::new());
    let mut command_buffer = TCommandBuffer::new(&device, &CommandBufferInfo::new());
    let mut swap_chain = TSwapChain::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );

    let vertex_shader_binary = include_bytes!("../outputs/resources/shaders/mandelbrot.vs.spv");
    let pixel_shader_binary = include_bytes!("../outputs/resources/shaders/mandelbrot.fs.spv");
    let shader = TShader::new(
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
    let vertex_state = TVertexState::new(&device, &vertex_state_info);

    let buffer_info = BufferInfo::new()
        .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
        .set_size(128);

    // 画面いっぱいに四角形を描く
    let vertex_buffer = TBuffer::new(&device, &buffer_info);
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

    let mut semaphore = TSemaphore::new(&device, &SemaphoreInfo::new());

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
