extern crate nalgebra_glm as glm;

use sjgfx::wgpu::*;
use sjgfx::*;
use sjgfx_interface::{GpuAccess, PrimitiveTopology, IndexFormat, ShaderInfo, VertexStateInfo, VertexBufferStateInfo, IBuffer, BufferInfo, IQueue, QueueInfo, ICommandBuffer};
use sjgfx_wgpu::{ShaderWgpu, VertexStateWgpu};
use winit::event::{WindowEvent, Event};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

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

fn load_obj<'a, TBuffer: IBuffer<'a>>(device: &'a TBuffer::Device) -> (TBuffer, TBuffer) {
    let vertex_buffer = TBuffer::new(&device, &BufferInfo::new().set_gpu_access_flags(GpuAccess::VERTEX_BUFFER).set_size(64));
    let index_buffer = TBuffer::new(&device, &BufferInfo::new().set_gpu_access_flags(GpuAccess::INDEX_BUFFER).set_size(64));

    (vertex_buffer, index_buffer)
}

#[allow(dead_code)]
fn make_command<'a, TQueue>(device: &'a TQueue::DeviceType, command_buffer: &mut TQueue::CommandBufferType)
where TQueue: IQueue<'a>
{
    let mut queue = TQueue::new(device, &QueueInfo::new());

    command_buffer.begin();
    command_buffer.enf();

    queue.execute(&command_buffer);
    queue.flush();
    queue.sync();
}

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let _device = DeviceBuilder::new().build();
    let device = DeviceBuilder::new().build_widh_surface(&window, &event_loop);
    let mut queue = QueueBuilder::new().build(&device);
    let mut command_buffer = CommandBufferBuilder::new().build(&device);

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

    let (vertex_buffer, index_buffer) =  load_obj(&device);

    let constant_buffer = BufferBuilder::new().set_gpu_access(GpuAccess::CONSTANT_BUFFER).set_size(std::mem::size_of::<ConstantBuffer>()).build(&device);
    constant_buffer.map_mut(|x: &mut ConstantBuffer| {
        let position = glm::vec3(1.5, 1.0, 3.0);
        let at = glm::vec3(0.0, 0.0, 0.0);
        let up = glm::vec3(0.0, 1.0, 0.0);
        let view_matrix: glm::Mat4x4 = glm::look_at(&position, &at, &up);
        let fov = std::f32::consts::PI / 4.0;
        let projection_matrix: glm::Mat4x4 = glm::perspective_fov(fov, 640.0, 480.0, 0.1, 100.0);

        x.pv = projection_matrix * view_matrix;
    });

    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<Vertex>() as i64)];
    let vertex_state = VertexStateWgpu::new(
        &device,
        &VertexStateInfo::new().set_buffer_state_info_array(vertex_buffer_state_info_array),
    );

    let mut swap_chain = SwapChainBuilder::new().build(&device);

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    let next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view(None, None);

                    command_buffer.begin();
                    command_buffer.set_render_targets([next_scan_buffer_view].into_iter(), None);
                    command_buffer.set_shader(&shader);
                    command_buffer.set_constant_buffer_address(0, constant_buffer.get_gpu_address());
                    command_buffer.set_vertex_state(&vertex_state);
                    command_buffer.set_vertex_buffer(0, &vertex_buffer);
                    command_buffer.draw_indexed(PrimitiveTopology::TriangleList, IndexFormat::Uint32, &index_buffer, 6, 0/*base_vertex*/);
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
