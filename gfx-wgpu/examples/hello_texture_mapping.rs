use sjgfx_interface::{DeviceInfo, QueueInfo, CommandBufferInfo, SwapChainInfo, SamplerInfo, TextureInfo, PrimitiveTopology, IndexFormat, ShaderInfo, BufferInfo, GpuAccess, ImageFormat};
use sjwgpu_wgpu::{DeviceWgpu, QueueWgpu, CommandBufferWgpu, SwapChainWgpu, SamplerWgpu, TextureWgpu, ShaderWgpu, BufferWgpu};
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{Event, WindowEvent}, platform::run_return::EventLoopExtRunReturn};

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

fn main()
{
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let device = DeviceWgpu::new_as_graphics(&DeviceInfo::new(), &window);
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
    let shader = ShaderWgpu::new(&device, &ShaderInfo::new().set_vertex_shader_binary(vertex_shader_binary.as_binary_u8()).set_pixel_shader_binary(pixel_shader_binary.as_binary_u8()));

    let texture = TextureWgpu::new(&device, &TextureInfo::new().set_width(640).set_height(480).set_gpu_access_flags(GpuAccess::TEXTURE).set_image_format(ImageFormat::R8G8B8A8Unorm));
    let sampler = SamplerWgpu::new(&device, &SamplerInfo::new());

    let mut vertex_buffer = BufferWgpu::new(&device, &BufferInfo::new().set_gpu_access_flags(GpuAccess::VERTEX_BUFFER).set_size(std::mem::size_of::<Vertex>() * 4));
    vertex_buffer.map_as_slice_mut(4, |x| {
        x[0] = Vertex{ x: -1.0, y: 1.0, u: 1.0, v: 0.0 };
        x[1] = Vertex{ x: -1.0, y: -1.0, u: 0.0, v: 0.0 };
        x[2] = Vertex{ x: 1.0, y: -1.0, u: 1.0, v: 0.0 };
        x[3] = Vertex{ x: 1.0, y: 1.0, u: 1.0, v: 1.0 };
    });

    let mut index_buffer = BufferWgpu::new(&device, &BufferInfo::new().set_gpu_access_flags(GpuAccess::INDEX_BUFFER).set_size(std::mem::size_of::<u32>() * 6));
    index_buffer.map_as_slice_mut(6, |x| {
        x[0] = 0;
        x[1] = 1;
        x[2] = 2;

        x[3] = 0;
        x[4] = 2;
        x[5] = 3;
    });

    let mut swap_chain = SwapChainWgpu::new(&device, &SwapChainInfo::new());

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow|{
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    let next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view();

                    command_buffer.begin();
                    command_buffer.set_render_targets([next_scan_buffer_view].into_iter(), None);
                    command_buffer.set_shader(&shader);
                    command_buffer.set_texture(0, &texture);
                    command_buffer.set_sampler(1, &sampler);
                    command_buffer.set_vertex_buffer(0, &vertex_buffer);
                    command_buffer.draw_indexed(PrimitiveTopology::TriangleList, IndexFormat::Uint32, &index_buffer, 6, 0/*base vertex*/);
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
