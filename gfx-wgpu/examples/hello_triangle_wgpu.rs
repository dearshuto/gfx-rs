use sjgfx_interface::{
    CommandBufferInfo, DeviceInfo, PrimitiveTopology, QueueInfo, ShaderInfo, SwapChainInfo,
    TextureArrayRange,
};
use sjgfx_wgpu::{CommandBufferWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu, SwapChainWgpu};
use wasm_bindgen::prelude::wasm_bindgen;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;

#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub fn print(log: &str) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        alert(log);
    }

    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", log);
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body| {
            body.append_child(&web_sys::Element::from(window.canvas()))
                .ok()
        })
        .expect("couldn't append canvas to document body");

    let mut device = DeviceWgpu::new_as_graphics(&DeviceInfo::new(), &window);
    let mut swap_chain = SwapChainWgpu::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );

    let mut queue = QueueWgpu::new(&mut device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    let mut compiler = sjgfx_util::ShaderCompiler::new();
    let vertex_shader_binary = compiler.create_binary(
        &include_str!("../../resources/examples/shaders/hello_bufferless_triangle.vs"),
        sjgfx_util::ShaderStage::Vertex,
    );
    let pixel_shader_binary = compiler.create_binary(
        &include_str!("../../resources/examples/shaders/hello_triangle.fs"),
        sjgfx_util::ShaderStage::Pixel,
    );
    let shader = ShaderWgpu::new(
        &mut device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(&vertex_shader_binary)
            .set_pixel_shader_binary(&pixel_shader_binary),
    );
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_size),
                ..
            } => {
                // TODO
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                // スキャンバッファの取得
                let mut next_scan_buffer_view =
                    swap_chain.acquire_next_scan_buffer_view(None, None);

                command_buffer.begin();
                command_buffer.clear_color(
                    &mut next_scan_buffer_view,
                    0.0,
                    0.1,
                    0.2,
                    1.0,
                    TextureArrayRange::new(),
                );
                command_buffer.set_shader(&shader);
                command_buffer.set_render_targets(&[&next_scan_buffer_view], None);
                command_buffer.draw(PrimitiveTopology::TriangleList, 3, 0);
                command_buffer.end();

                queue.execute(&command_buffer);

                // 結果の表示
                queue.present(&mut swap_chain);

                queue.sync();
            }
            _ => {}
        }
    });
}
