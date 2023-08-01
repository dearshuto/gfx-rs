use std::sync::Arc;

use eframe::CreationContext;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native("", options, Box::new(|cc| Box::new(App::new(cc)))).unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    // eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let mut web_options = eframe::WebOptions::default();
    web_options.wgpu_options.supported_backends = wgpu::Backend::BrowserWebGpu.into();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "canvas", // hardcode it
                web_options,
                Box::new(|cc| Box::new(App::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

struct App {
    renderer: eframe::Renderer,
}

impl App {
    pub fn new(context: &CreationContext) -> Self {
        if let Some(render_state) = &context.wgpu_render_state {
            let device = render_state.device.clone();
            let demo_manager = demolib::DemoManager::new(device.clone());
            let _ = context
                .wgpu_render_state
                .as_ref()
                .unwrap()
                .renderer
                .write()
                .paint_callback_resources
                .insert(demo_manager);
            Self {
                renderer: eframe::Renderer::Wgpu,
            }
        } else {
            Self {
                renderer: eframe::Renderer::Glow,
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::SidePanel::left("Demo List").show(ctx, |ui| {
            ui.label("Triangle");
        });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            eframe::egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let (rect, _response) = ui.allocate_exact_size(
                    eframe::egui::vec2(400.0, 400.0),
                    eframe::egui::Sense::drag(),
                );

                let callback = match self.renderer {
                    eframe::Renderer::Glow => eframe::egui::PaintCallback {
                        rect,
                        callback: Arc::new(eframe::egui_glow::CallbackFn::new(
                            move |_info, _painter| {},
                        )),
                    },
                    eframe::Renderer::Wgpu => {
                        let function = eframe::egui_wgpu::CallbackFn::new()
                            .prepare(move |_device, _queue, _command_encoder, render_resources| {
                                let demo_manager: &mut demolib::DemoManager =
                                    render_resources.get_mut().unwrap();
                                demo_manager.update();
                                Vec::default()
                            })
                            .paint(move |_info, render_pass, render_resources| {
                                let demo_manager: &demolib::DemoManager =
                                    render_resources.get().unwrap();
                                demo_manager.draw(render_pass);
                            });
                        eframe::egui::PaintCallback {
                            rect,
                            callback: Arc::new(function),
                        }
                    }
                };

                ui.painter().add(callback);
            });
        });
    }
}
