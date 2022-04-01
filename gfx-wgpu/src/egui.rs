use std::{sync::Arc, time::Instant};

use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::App;
use winit::{event::Event, event_loop::EventLoop};

use crate::{CommandBufferWgpu, DeviceWgpu};

pub struct Interopebility {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    platform: Platform,
    egui_render_pass: RenderPass,
    demo_app: egui_demo_lib::WrapApp,
    repaint_signal: Arc<ExampleRepaintSignal>,

    start_time: Instant,
}

impl Interopebility {
    pub fn new(device: &DeviceWgpu, event_loop: &EventLoop<()>) -> Self {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: 1600,
            physical_height: 1200,
            scale_factor: 2.0,
            font_definitions: egui::FontDefinitions::default(),
            style: Default::default(),
        });

        let queue = device.clone_queue();
        let device = device.close_device();
        let egui_render_pass = RenderPass::new(&device, wgpu::TextureFormat::Bgra8UnormSrgb, 1);

        let demo_app = egui_demo_lib::WrapApp::default();

        let repaint_signal = std::sync::Arc::new(ExampleRepaintSignal(std::sync::Mutex::new(
            event_loop.create_proxy(),
        )));

        Self {
            device,
            queue,
            platform,
            egui_render_pass,
            demo_app,
            repaint_signal,
            start_time: Instant::now(),
        }
    }

    pub fn update(&mut self, event: &Event<()>) {
        self.platform.handle_event(event);
    }

    pub fn push_draw_command(&mut self, command_buffer: &mut CommandBufferWgpu) {
        self.platform
            .update_time(self.start_time.elapsed().as_secs_f64());
        self.platform.begin_frame();

        let color_attachment = command_buffer.get_color_target_view();
        let mut frame = epi::Frame::new(epi::backend::FrameData {
            info: epi::IntegrationInfo {
                name: "egui_example",
                web_info: None,
                cpu_usage: None,
                native_pixels_per_point: None,
                prefer_dark_mode: None,
            },
            output: epi::backend::AppOutput::default(),
            repaint_signal: self.repaint_signal.clone(),
        });

        self.demo_app.update(&self.platform.context(), &mut frame);
        let (_output, paint_commands) = self.platform.end_frame(None);
        let paint_jobs = self.platform.context().tessellate(paint_commands);

        let screen_descriptor = ScreenDescriptor {
            physical_width: 1600,
            physical_height: 1200,
            scale_factor: 2.0,
        };
        self.egui_render_pass.update_texture(
            &self.device,
            &self.queue,
            &self.platform.context().font_image(),
        );
        self.egui_render_pass
            .update_user_textures(&self.device, &self.queue);
        self.egui_render_pass.update_buffers(
            &self.device,
            &self.queue,
            &paint_jobs,
            &screen_descriptor,
        );

        // 描画コマンドを作成
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        self.egui_render_pass
            .execute(
                &mut encoder,
                color_attachment,
                &paint_jobs,
                &screen_descriptor,
                None, /*clear_color*/
            )
            .unwrap();

        self.queue.submit(Some(encoder.finish()));
    }
}

struct ExampleRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<()>>);
impl epi::backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(()).ok();
    }
}
