use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, PrimitiveTopology,
    QueueInfo, ShaderInfo, SwapChainInfo, TextureArrayRange, VertexAttributeStateInfo,
    VertexBufferStateInfo, VertexStateInfo,
};
use sjgfx_wgpu::{
    BufferWgpu, CommandBufferWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu, SwapChainWgpu,
    VertexStateWgpu,
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
}

#[repr(C)]
#[derive(Debug, Default)]
struct ConstantBuffer {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub _padding: f32,
}

pub fn main() {
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
        &include_str!("../../resources/examples/shaders/hello_constant_buffer.vs"),
        sjgfx_util::ShaderStage::Vertex,
    );
    let pixel_shader_binary = compiler.create_binary(
        &include_str!("../../resources/examples/shaders/hello_constant_buffer.fs"),
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
        .set_format(AttributeFormat::Float32_32)
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
            .set_size(std::mem::size_of::<Vertex>() * 3),
    );
    vertex_buffer.map_as_slice_mut(3, |x| {
        x[0] = Vertex { x: -0.5, y: -0.5 };
        x[1] = Vertex { x: 0.5, y: -0.5 };
        x[2] = Vertex { x: 0.0, y: 0.5 };
    });

    let constant_buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER)
            .set_size(std::mem::size_of::<ConstantBuffer>()),
    );

    let mut swap_chain = SwapChainWgpu::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );

    let mut frame = 0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        frame = (frame + 1) % 360;

        constant_buffer.map_mut(|x: &mut ConstantBuffer| {
            x.red = ((frame as f32).to_radians().sin() + 1.0) * 0.5;
            x.green = 0.5;
            x.blue = 0.1;
        });

        window.request_redraw();

        match event {
            Event::RedrawRequested(_) => {
                let color_target_view = swap_chain.acquire_next_scan_buffer_view(None, None);

                command_buffer.begin();
                command_buffer.clear_color(
                    color_target_view,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    TextureArrayRange::new(),
                );
                command_buffer.set_render_targets(&[color_target_view], None);
                command_buffer.set_shader(&shader);
                command_buffer.set_constant_buffer(0, &constant_buffer);
                command_buffer.set_vertex_state(&vertex_state);
                command_buffer.set_vertex_buffer(0, &vertex_buffer);
                command_buffer.draw(
                    PrimitiveTopology::TriangleList,
                    3, /*coount*/
                    0, /*offset*/
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
