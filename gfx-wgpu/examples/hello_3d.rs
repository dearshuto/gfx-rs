extern crate nalgebra_glm as glm;

use std::{thread::sleep, time::Duration};

use sjgfx_interface::{
    BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, PrimitiveTopology, QueueInfo, ShaderInfo,
    SwapChainInfo, VertexBufferStateInfo, VertexStateInfo,
};
use sjgfx_wgpu::{
    BufferWgpu, CommandBufferWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu, SwapChainWgpu,
    VertexStateWgpu,
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

#[repr(C)]
struct ConstantBuffer {
    pv: glm::Mat4x4,
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
            &include_str!("../../resources/examples/shaders/hello_3d.vs"),
            shaderc::ShaderKind::Vertex,
            "vs.glsl",
            "main",
            None,
        )
        .unwrap();
    let pixel_shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_3d.fs"),
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

    let vertex_buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(std::mem::size_of::<Vertex>() * 6),
    );
    vertex_buffer.map_as_slice_mut(6, |x| {
        x[0] = Vertex {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        };
        x[1] = Vertex {
            x: 0.5,
            y: 0.0,
            z: 0.0,
        };
        x[2] = Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.5,
        };

        x[3] = Vertex {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        };
        x[4] = Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.5,
        };
        x[5] = Vertex {
            x: -0.5,
            y: 0.0,
            z: 0.0,
        };
    });

    let constant_buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER)
            .set_size(std::mem::size_of::<ConstantBuffer>()),
    );
    constant_buffer.map_mut(|x: &mut ConstantBuffer| {
        let position = glm::vec3(1.5, 1.0, 3.0);
        let at = glm::vec3(0.0, 0.0, 0.0);
        let up = glm::vec3(0.0, 1.0, 0.0);
        let view_matrix: glm::Mat4x4 = glm::look_at(&position, &at, &up);
        let fov = std::f32::consts::PI / 4.0;
        let projection_matrix: glm::Mat4x4 = glm::perspective_fov(fov, 640.0, 480.0, 0.1, 100.0);

        x.pv = projection_matrix * view_matrix;
    });

    let mut swap_chain = SwapChainWgpu::new(&device, &SwapChainInfo::new());

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    let color_target_view = swap_chain.acquire_next_scan_buffer_view(None, None);

                    command_buffer.begin();
                    command_buffer.set_render_targets([color_target_view].into_iter(), None);
                    command_buffer.set_shader(&shader);
                    command_buffer.set_constant_buffer(0, &constant_buffer);
                    command_buffer.set_vertex_state(&vertex_state);
                    command_buffer.set_vertex_buffer(0, &vertex_buffer);
                    command_buffer.draw(
                        PrimitiveTopology::TriangleList,
                        6, /*coount*/
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
                    should_close = true;
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
        });

        sleep(Duration::from_millis(16));
    }
}
