use std::{thread::sleep, time::Duration};

use sjgfx_interface::{DeviceInfo, CommandBufferInfo, SwapChainInfo, QueueInfo};
use sjwgpu_wgpu::{DeviceWgpu, CommandBufferWgpu, SwapChainWgpu, QueueWgpu};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

// iced でカスタム描画ができないか試してみた。
// iced 内部で作られた Device にアクセスする方法がよくわからない
fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut device = DeviceWgpu::new_as_graphics(&DeviceInfo::new(), &window);
    let _backend = iced_wgpu::Backend::new(device.get_device_mut(), iced_wgpu::settings::Settings::default(), wgpu::TextureFormat::Rgba8Unorm);

    let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());
    let mut swap_chain = SwapChainWgpu::new(&device, &SwapChainInfo::new());

    let _staging_belt = wgpu::util::StagingBelt::new(128);

    let _renderer : Option<iced_wgpu::Renderer<iced_wgpu::Backend>> = None;

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    let next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view();

                    command_buffer.begin();
                    command_buffer.set_render_targets([next_scan_buffer_view].into_iter(), None);
                    command_buffer.end();

                    // let image_view = next_scan_buffer_view.get_texture_view();
                    // backend.present(device.get_device(), &staging_belt, encoder, image_view, &[], viewport, &[]);

                    queue.execute(&command_buffer);
                    queue.present(&mut swap_chain);
                    queue.flush();
                    queue.sync();
                },
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

        sleep(Duration::from_millis(16));
    }
}
