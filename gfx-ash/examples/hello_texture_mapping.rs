use sjgfx_ash::{DeviceAsh, CommandBufferAsh, QueueAsh, SwapChainAsh, SemaphoreAsh};
use sjgfx_interface::{DeviceInfo, CommandBufferInfo, QueueInfo, SwapChainInfo};
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, platform::run_return::EventLoopExtRunReturn};

fn main()
{
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_resizable(false).build(&event_loop).unwrap();
    let mut device = DeviceAsh::new_with_surface(&DeviceInfo::new(), &window);
    let mut swap_chain = SwapChainAsh::new(&mut device, &SwapChainInfo::new());
    let mut queue = QueueAsh::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferAsh::new(&device, &CommandBufferInfo::new());

    let mut semaphore = SemaphoreAsh::new(&device);
    event_loop.run_return(|event, _, control_flow|{
        *control_flow = ControlFlow::Wait;

        let next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view(Some(&mut semaphore), None);

        command_buffer.begin();
        //command_buffer.set_render_targets([next_scan_buffer_view].into_iter(), None);
        command_buffer.end();

        queue.execute(&command_buffer);
        queue.present(&mut swap_chain);
        queue.flush();
        queue.sync();
    });
}
