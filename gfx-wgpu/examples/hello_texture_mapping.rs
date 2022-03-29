use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, ImageFormat,
    IndexFormat, PrimitiveTopology, QueueInfo, SamplerInfo, ShaderInfo, SwapChainInfo, TextureInfo,
    TextureViewInfo, VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};
use sjgfx_wgpu::{
    BufferWgpu, CommandBufferWgpu, DeviceWgpu, QueueWgpu, SamplerWgpu, ShaderWgpu, SwapChainWgpu,
    TextureViewWgpu, TextureWgpu, VertexStateWgpu,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

#[repr(C)]
struct Vertex {
    #[allow(dead_code)]
    pub x: f32,

    #[allow(dead_code)]
    pub y: f32,

    #[allow(dead_code)]
    pub u: f32,

    #[allow(dead_code)]
    pub v: f32,
}

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut device = DeviceWgpu::new_as_graphics(&DeviceInfo::new(), &window);
    let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    let mut compiler = shaderc::Compiler::new().unwrap();
    let vertex_shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_texture_mapping.vs"),
            shaderc::ShaderKind::Vertex,
            "vs.glsl",
            "main",
            None,
        )
        .unwrap();
    let pixel_shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_texture_mapping.fs"),
            shaderc::ShaderKind::Fragment,
            "fs.glsl",
            "main",
            None,
        )
        .unwrap();
    let shader = ShaderWgpu::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(vertex_shader_binary.as_binary_u8())
            .set_pixel_shader_binary(pixel_shader_binary.as_binary_u8()),
    );

    let attribute_state_info_array = [
        VertexAttributeStateInfo::new()
            .set_buffer_index(0)
            .set_format(AttributeFormat::Float32_32)
            .set_offset(0)
            .set_slot(0),
        VertexAttributeStateInfo::new()
            .set_buffer_index(0)
            .set_format(AttributeFormat::Float32_32)
            .set_offset((std::mem::size_of::<f32>() * 2) as i64)
            .set_slot(1),
    ];
    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<Vertex>() as i64)];
    let vertex_state = VertexStateWgpu::new(
        &device,
        &VertexStateInfo::new()
            .set_attribute_state_info_array(attribute_state_info_array.into_iter())
            .set_buffer_state_info_array(vertex_buffer_state_info_array),
    );

    let buffer = include_bytes!("../../resources/examples/images/lena_color.png");
    let image = image::load_from_memory_with_format(buffer, image::ImageFormat::Png).unwrap();

    let texture = TextureWgpu::new_with_data(
        &device,
        &TextureInfo::new()
            .set_width(512)
            .set_height(512)
            .set_gpu_access_flags(GpuAccess::TEXTURE)
            .set_image_format(ImageFormat::R8G8B8Unorm),
        image.as_bytes(),
    );
    let texture_view = TextureViewWgpu::new(
        &device,
        &TextureViewInfo::new().set_format(ImageFormat::R8G8B8Unorm),
        &texture,
    );
    let sampler = SamplerWgpu::new(&device, &SamplerInfo::new());

    let vertex_buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(std::mem::size_of::<f32>() * 16),
    );
    vertex_buffer.map_as_slice_mut(16, |x: &mut [f32]| {
        x[0] = -1.0;
        x[1] = 1.0;
        x[2] = 0.0;
        x[3] = 1.0;

        x[4] = -1.0;
        x[5] = -1.0;
        x[6] = 0.0;
        x[7] = 0.0;

        x[8] = 1.0;
        x[9] = -1.0;
        x[10] = 1.0;
        x[11] = 0.0;

        x[12] = 1.0;
        x[13] = 1.0;
        x[14] = 1.0;
        x[15] = 1.0;
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
        &SwapChainInfo::new().with_width(960).with_height(960),
    );

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    let next_scan_buffer_view =
                        swap_chain.acquire_next_scan_buffer_view(None, None);

                    command_buffer.begin();
                    command_buffer.set_render_targets([next_scan_buffer_view].into_iter(), None);
                    command_buffer.set_shader(&shader);
                    command_buffer.set_texture(0, &texture_view);
                    command_buffer.set_sampler(1, &sampler);
                    command_buffer.set_vertex_state(&vertex_state);
                    command_buffer.set_vertex_buffer(0, &vertex_buffer);
                    command_buffer.draw_indexed(
                        PrimitiveTopology::TriangleList,
                        IndexFormat::Uint32,
                        &index_buffer,
                        6,
                        0, /*base vertex*/
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
                    should_close = true;
                }
                _ => {}
            }
        });
    }
}
