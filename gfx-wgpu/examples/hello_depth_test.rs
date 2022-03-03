use sjgfx_interface::{
    BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, ImageFormat, IndexFormat,
    PrimitiveTopology, QueueInfo, ShaderInfo, SwapChainInfo, TextureInfo, VertexBufferStateInfo,
    VertexStateInfo,
};
use sjwgpu_wgpu::{
    BufferWgpu, CommandBufferWgpu, DepthStencilViewWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu,
    SwapChainWgpu, TextureWgpu, VertexStateWgpu,
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
    pub z: f32,
}

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let device = DeviceWgpu::new_as_graphics(&DeviceInfo::new(), &window);
    let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    let mut compiler = shaderc::Compiler::new().unwrap();
    let vertex_shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_depth_test.vs"),
            shaderc::ShaderKind::Vertex,
            "vs.glsl",
            "main",
            None,
        )
        .unwrap();
    let pixel_shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_depth_test.fs"),
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

    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<Vertex>() as i64)];
    let vertex_state = VertexStateWgpu::new(
        &device,
        &VertexStateInfo::new().set_buffer_state_info_array(vertex_buffer_state_info_array),
    );

    let mut vertex_buffer = BufferWgpu::new(
        &device,
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

    let mut index_buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::INDEX_BUFFER)
            .set_size(std::mem::size_of::<u32>() * 6),
    );
    index_buffer.map_as_slice_mut::<u32>(6, |buffer| {
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
            .set_width(640)
            .set_height(480)
            .set_image_format(ImageFormat::D32)
            .set_gpu_access_flags(GpuAccess::DEPTH_STENCIL),
    );
    let depth_stencil_view = DepthStencilViewWgpu::new(&device, &texture);

    let mut swap_chain = SwapChainWgpu::new(&device, &SwapChainInfo::new());

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    let color_target_view = swap_chain.acquire_next_scan_buffer_view();

                    command_buffer.begin();
                    command_buffer.set_render_targets(
                        [color_target_view].into_iter(),
                        Some(&depth_stencil_view),
                    );
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
                    should_close = true;
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
        });
    }
}
