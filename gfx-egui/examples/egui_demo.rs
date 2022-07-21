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
use sjvi::{IDisplay, IInstance};

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
    let mut instance = TApi::Instance::new();
    let id = instance.create_display();
    let display = instance.try_get_display(&id).unwrap();

    let mut device = TDeviceBuilder::<TApi>::new().build_with_surface(&display);
    let mut queue = TQueueBuilder::<TApi>::new().build(&mut device);
    let mut swap_chain = TSwapChainBuilder::<TApi>::new()
        .with_width(2560)
        .with_height(1920)
        .build(&mut device);
    let mut command_buffer = TCommandBufferBuilder::<TApi>::new().build(&device);
    let mut semaphore = TSemaphoreBuilder::<TApi>::new().build(&device);

    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: 1280,
        physical_height: 960,
        scale_factor: display.get_scale_factor(),
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    let mut demo_app = egui_demo_lib::WrapApp::default();
    let mut gfx_egui_render_pass = gfx_egui::RenderPass::<TApi>::new(&mut device);

    let start_time = Instant::now();
    let mut previous_frame_time = None;
    // let repaint_signal = std::sync::Arc::new(ExampleRepaintSignal(std::sync::Mutex::new(
    //     event_loop.create_proxy(),
    // )));
    let repaint_signal = std::sync::Arc::new(ExampleRepaintSignal {});
    while instance.try_update() {
        let display = instance.try_get_display(&id).unwrap();
        if display.is_redraw_requested() {
            // 画面をクリア
            let mut next_scan_buffer_view =
                swap_chain.acquire_next_scan_buffer_view(Some(&mut semaphore), None /*fence*/);
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
                    native_pixels_per_point: Some(display.get_scale_factor() as _),
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

            gfx_egui_render_pass.update_buffers(&mut device, &paint_jobs);
            gfx_egui_render_pass.update_texture(&mut device, &platform.context().font_image());
            gfx_egui_render_pass.execute(&next_scan_buffer_view, &mut queue, &paint_jobs);

            queue.present(&mut swap_chain);

            queue.flush();
            queue.sync();
        }
    }
}
