use sjgfx_interface::{CommandBufferInfo, DeviceInfo, QueueInfo, SwapChainInfo};
use sjgfx_wgpu::{CommandBufferWgpu, DeviceWgpu, QueueWgpu, SwapChainWgpu};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&mut event_loop).unwrap();

    println!(
        "{}, {}",
        window.inner_size().width,
        window.inner_size().height
    );
    println!("{}", window.scale_factor());

    let mut device = DeviceWgpu::new_as_graphics(&DeviceInfo::new(), &window);
    let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    let mut swap_chain = SwapChainWgpu::new(&mut device, &SwapChainInfo::new());
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    let size = window.inner_size();
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: device
            .get_surface()
            .get_preferred_format(&device.get_adapter())
            .unwrap(),
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Fifo,
    };
    device
        .get_surface()
        .configure(&device.get_device(), &surface_config);
    let mut interopebility = sjgfx_wgpu::egui::Interopebility::new(&device, &event_loop);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        interopebility.update(&event);
        match event {
            winit::event::Event::RedrawRequested(..) => {
                let scan_buffer = swap_chain.acquire_next_scan_buffer_view(None, None);

                command_buffer.begin();
                command_buffer.set_render_targets([scan_buffer].into_iter(), None);
                interopebility.push_draw_command(&mut command_buffer);
                command_buffer.end();

                queue.execute(&command_buffer);
                queue.present(&mut swap_chain);
                queue.flush();
                queue.sync();
            }
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
