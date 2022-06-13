use std::time::Instant;

use egui::FontDefinitions;
use egui_winit_platform::Platform;
use egui_winit_platform::PlatformDescriptor;
use epi::App;
use sjgfx::{
    api::IApi, TCommandBufferBuilder, TDeviceBuilder, TQueueBuilder, TSemaphoreBuilder,
    TSwapChainBuilder,
};
use sjgfx_interface::{ICommandBuffer, IQueue, ISwapChain, TextureArrayRange};
use winit::event::Event::RedrawRequested;
use winit::event::Event::WindowEvent;
use winit::event::WindowEvent::Resized;
use winit::{
    dpi::PhysicalSize,
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

struct ExampleRepaintSignal;

impl epi::backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        //self.0.lock().unwrap().send_event(self.).ok();
    }
}

fn main() {
    run::<sjgfx::api::Wgpu>();
}

fn run<TApi: IApi>() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(2560, 1920))
        .build(&event_loop)
        .unwrap();

    let mut device = TDeviceBuilder::<TApi>::new().build_with_surface(&window, &event_loop);
    let mut queue = TQueueBuilder::<TApi>::new().build(&device);
    let mut swap_chain = TSwapChainBuilder::<TApi>::new()
        .with_width(2560)
        .with_height(1920)
        .build(&mut device);
    let mut command_buffer = TCommandBufferBuilder::<TApi>::new().build(&device);
    let mut semaphore = TSemaphoreBuilder::<TApi>::new().build(&device);

    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: 1280,
        physical_height: 960,
        scale_factor: window.scale_factor(),
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    let mut demo_app = egui_demo_lib::WrapApp::default();
    let mut gfx_egui_render_pass = gfx_egui::RenderPass::<TApi>::new(&device);

    let start_time = Instant::now();
    let mut previous_frame_time = None;
    // let repaint_signal = std::sync::Arc::new(ExampleRepaintSignal(std::sync::Mutex::new(
    //     event_loop.create_proxy(),
    // )));
    let repaint_signal = std::sync::Arc::new(ExampleRepaintSignal {});
    event_loop.run_return(|event, _, control_flow| {
        platform.handle_event(&event);

        *control_flow = ControlFlow::Poll;

        window.request_redraw();
        match event {
            RedrawRequested(..) => {
                // 画面をクリア
                let mut next_scan_buffer_view = swap_chain
                    .acquire_next_scan_buffer_view(Some(&mut semaphore), None /*fence*/);
                command_buffer.begin();
                command_buffer.clear_color(
                    &mut next_scan_buffer_view,
                    0.1,
                    0.2,
                    0.4,
                    0.0,
                    TextureArrayRange::new(),
                );
                command_buffer.end();
                queue.execute(&command_buffer);

                platform.update_time(start_time.elapsed().as_secs_f64());

                let egui_start = Instant::now();
                platform.begin_frame();
                let app_output = epi::backend::AppOutput::default();

                let mut frame = epi::Frame::new(epi::backend::FrameData {
                    info: epi::IntegrationInfo {
                        name: "egui_example",
                        web_info: None,
                        cpu_usage: previous_frame_time,
                        native_pixels_per_point: Some(window.scale_factor() as _),
                        prefer_dark_mode: None,
                    },
                    output: app_output,
                    repaint_signal: repaint_signal.clone(),
                });

                // Draw the demo application.
                demo_app.update(&platform.context(), &mut frame);
                let (_output, paint_commands) = platform.end_frame(Some(&window));
                let paint_jobs = platform.context().tessellate(paint_commands);

                let frame_time = (Instant::now() - egui_start).as_secs_f64() as f32;
                previous_frame_time = Some(frame_time);

                gfx_egui_render_pass.update_buffers(&device, &paint_jobs);
                gfx_egui_render_pass.update_texture(&device, &platform.context().font_image());
                gfx_egui_render_pass.execute(&next_scan_buffer_view, &mut queue, &paint_jobs);

                queue.present(&mut swap_chain);

                queue.flush();
                queue.sync();
            }
            WindowEvent { event, .. } => match event {
                Resized(_size) => {}
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        }
    });
}
