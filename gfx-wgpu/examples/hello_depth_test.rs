use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, ImageFormat,
    IndexFormat, PrimitiveTopology, QueueInfo, ShaderInfo, SwapChainInfo, TextureArrayRange,
    TextureInfo, VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};
use sjgfx_wgpu::{
    BufferWgpu, CommandBufferWgpu, DepthStencilViewWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu,
    SwapChainWgpu, TextureWgpu, VertexStateWgpu,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;

#[repr(C)]
#[derive(Debug, Default, Clone)]
struct Vertex {
    #[allow(dead_code)]
    pub x: f32,

    #[allow(dead_code)]
    pub y: f32,

    #[allow(dead_code)]
    pub z: f32,
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

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
    let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    let mut compiler = sjgfx_util::ShaderCompiler::new();
    let vertex_shader_binary = compiler.create_binary(
        &include_str!("../../resources/examples/shaders/hello_depth_test.vs"),
        sjgfx_util::ShaderStage::Vertex,
    );
    let pixel_shader_binary = compiler.create_binary(
        &include_str!("../../resources/examples/shaders/hello_depth_test.fs"),
        sjgfx_util::ShaderStage::Pixel,
    );
    let shader = ShaderWgpu::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(&vertex_shader_binary)
            .set_pixel_shader_binary(&pixel_shader_binary),
    );

    let attribute_state_info_array = [VertexAttributeStateInfo::new()
        .set_buffer_index(0)
        .set_format(AttributeFormat::Float32_32_32)
        .set_offset(0)
        .set_slot(0)];
    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<Vertex>() as i64)];
    let vertex_state = VertexStateWgpu::new(
        &device,
        &VertexStateInfo::new()
            .set_attribute_state_info_array(attribute_state_info_array.into_iter())
            .set_buffer_state_info_array(vertex_buffer_state_info_array),
    );

    let vertex_buffer = BufferWgpu::new(
        &mut device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(std::mem::size_of::<Vertex>() * 6),
    );
    vertex_buffer.map_as_slice_mut(6, |buffer| {
        buffer[0] = Vertex {
            x: -0.5,
            y: -0.5,
            z: 0.5,
        };
        buffer[1] = Vertex {
            x: 0.5,
            y: -0.5,
            z: 0.5,
        };
        buffer[2] = Vertex {
            x: 0.0,
            y: 0.5,
            z: 0.5,
        };

        buffer[3] = Vertex {
            x: -0.3,
            y: 0.2,
            z: 1.0,
        };
        buffer[4] = Vertex {
            x: 0.2,
            y: -0.6,
            z: 0.0,
        };
        buffer[5] = Vertex {
            x: 0.4,
            y: 0.6,
            z: 0.8,
        };
    });

    let index_buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::INDEX_BUFFER)
            .set_size(std::mem::size_of::<u32>() * 6),
    );
    index_buffer.map_as_slice_mut(6, |buffer| {
        buffer[0] = 0;
        buffer[1] = 1;
        buffer[2] = 2;

        buffer[3] = 3;
        buffer[4] = 4;
        buffer[5] = 5;
    });

    // 深度バッファ
    let texture = TextureWgpu::new(
        &device,
        &TextureInfo::new()
            .set_width(1280)
            .set_height(960)
            .set_image_format(ImageFormat::D32)
            .set_gpu_access_flags(GpuAccess::DEPTH_STENCIL),
    );
    let depth_stencil_view = DepthStencilViewWgpu::new(&device, &texture);

    let mut swap_chain = SwapChainWgpu::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                let color_target_view = swap_chain.acquire_next_scan_buffer_view(None, None);

                command_buffer.begin();
                command_buffer.clear_color(
                    color_target_view,
                    0.1,
                    0.2,
                    0.3,
                    1.0,
                    TextureArrayRange::new(),
                );
                command_buffer.set_render_targets(&[&color_target_view], Some(&depth_stencil_view));
                command_buffer.set_shader(&shader);
                command_buffer.set_vertex_state(&vertex_state);
                command_buffer.set_vertex_buffer(0, &vertex_buffer);
                command_buffer.draw_indexed(
                    PrimitiveTopology::TriangleList,
                    IndexFormat::Uint32,
                    &index_buffer,
                    6, /*index count*/
                    0, /*base_index*/
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
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
