use std::{thread::sleep, time::Duration};

use sjgfx_interface::{DeviceInfo, QueueInfo, CommandBufferInfo, VertexBufferStateInfo, BufferInfo, VertexStateInfo, GpuAccess, ShaderInfo, SwapChainInfo, PrimitiveTopology};
use sjwgpu_wgpu::{DeviceWgpu, QueueWgpu, CommandBufferWgpu, VertexStateWgpu, BufferWgpu, ShaderWgpu, SwapChainWgpu};
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, platform::run_return::EventLoopExtRunReturn, event::{Event, WindowEvent}};

#[repr(C)]
struct Vertex {
    #[allow(dead_code)]
    pub x: f32,

    #[allow(dead_code)]
    pub y: f32,
}

#[repr(C)]
struct ConstantBuffer {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub _padding: f32,
}

pub fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let device = DeviceWgpu::new_as_graphics(&DeviceInfo::new(), &window);
    let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    let mut compiler = shaderc::Compiler::new().unwrap();
    let vertex_shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_constant_buffer.vs"),
            shaderc::ShaderKind::Vertex,
            "vs.glsl",
            "main",
            None,
        )
        .unwrap();
    let pixel_shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_constant_buffer.fs"),
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
            .set_size(std::mem::size_of::<Vertex>() * 3),
    );
    vertex_buffer.map_as_slice_mut::<Vertex>(3, |x| {
        x[0] = Vertex{ x: -0.5, y: -0.5 };
        x[1] = Vertex{ x: 0.5, y: -0.5 };
        x[2] = Vertex{ x: 0.0, y: 0.5 };
    });

    let mut constant_buffer = BufferWgpu::new(&device, &BufferInfo::new().set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER).set_size(std::mem::size_of::<ConstantBuffer>()));
    constant_buffer.map_mut::<ConstantBuffer>(|x| {
        x.red = 1.0;
        x.green = 0.5;
        x.blue = 0.1;
    });

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
                        None,
                    );
                    command_buffer.set_shader(&shader);
                    command_buffer.set_constant_buffer_address(0, constant_buffer.get_gpu_address());
                    command_buffer.set_vertex_state(&vertex_state);
                    command_buffer.set_vertex_buffer(0, &vertex_buffer);
                    command_buffer.draw(PrimitiveTopology::TriangleList, 3/*coount*/, 0/*offset*/);
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

        sleep(Duration::from_millis(16));
    }
}
