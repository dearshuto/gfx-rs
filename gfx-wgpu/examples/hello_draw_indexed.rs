use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IndexFormat,
    PrimitiveTopology, QueueInfo, ShaderInfo, SwapChainInfo, TextureArrayRange,
    VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};
use sjgfx_wgpu::{
    BufferWgpu, CommandBufferWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu, SwapChainWgpu,
    VertexStateWgpu,
};
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[repr(C)]
#[derive(Default, Clone)]
struct Vertex {
    #[allow(dead_code)]
    pub x: f32,

    #[allow(dead_code)]
    pub y: f32,
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

    // シェーダ
    let mut compiler = sjgfx_util::ShaderCompiler::new();
    let vertex_shader_binary = compiler.create_binary(
        &include_str!("../../resources/examples/shaders/hello_triangle.vs"),
        sjgfx_util::ShaderStage::Vertex,
    );
    let pixel_shader_binary = compiler.create_binary(
        &include_str!("../../resources/examples/shaders/hello_triangle.fs"),
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
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(std::mem::size_of::<Vertex>() * 4),
    );
    vertex_buffer.map_as_slice_mut(4, |x: &mut [Vertex]| {
        x[0] = Vertex { x: -0.5, y: 0.5 };
        x[1] = Vertex { x: -0.5, y: -0.5 };
        x[2] = Vertex { x: 0.5, y: -0.5 };
        x[3] = Vertex { x: 0.5, y: 0.5 };
    });

    let index_buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::INDEX_BUFFER)
            .set_size(std::mem::size_of::<u32>() * 6),
    );
    index_buffer.map_as_slice_mut(6, |x| {
        x[0] = 0;
        x[1] = 1;
        x[2] = 2;
        x[3] = 0;
        x[4] = 2;
        x[5] = 3;
    });

    let mut swap_chain = SwapChainWgpu::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                let mut color_target_view = swap_chain.acquire_next_scan_buffer_view(None, None);

                command_buffer.begin();
                command_buffer.clear_color(
                    &mut color_target_view,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    TextureArrayRange::new(),
                );
                command_buffer.set_render_targets(&[&color_target_view], None);
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
