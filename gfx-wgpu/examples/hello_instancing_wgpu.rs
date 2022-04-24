extern crate nalgebra_glm as glm;

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
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

#[repr(C)]
struct TransformData {
    positions: [glm::Vec4; 9],
}

fn main() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1280, 960))
        .build(&event_loop)
        .unwrap();
    let mut device = DeviceWgpu::new_as_graphics(&DeviceInfo::new(), &window);
    let mut swap_chain = SwapChainWgpu::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );
    let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    let mut compiler = sjgfx_util::ShaderCompiler::new();
    let vertex_shader_binary = compiler.create_binary(
        &include_str!("resources/shaders/hello_instancing.vs"),
        sjgfx_util::ShaderStage::Vertex,
    );
    let pixel_shader_binary = compiler.create_binary(
        &include_str!("resources/shaders/hello_instancing.fs"),
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
        [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<glm::Vec2>() as i64)];
    let vertex_state = VertexStateWgpu::new(
        &device,
        &VertexStateInfo::new()
            .set_attribute_state_info_array(attribute_state_info_array.into_iter())
            .set_buffer_state_info_array(vertex_buffer_state_info_array),
    );

    // PV 行列
    // 位置情報
    let positions_buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_size(std::mem::size_of::<TransformData>())
            .set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER),
    );
    positions_buffer.map_mut(|x: &mut TransformData| {
        x.positions[0] = glm::vec4(-0.75, -0.75, 0.0, 0.0);
        x.positions[1] = glm::vec4(0.0, -0.75, 0.0, 0.0);
        x.positions[2] = glm::vec4(0.75, -0.75, 0.0, 0.0);

        x.positions[3] = glm::vec4(-0.75, 0.0, 0.0, 0.0);
        x.positions[4] = glm::vec4(0.0, 0.0, 0.0, 0.0);
        x.positions[5] = glm::vec4(0.75, 0.0, 0.0, 0.0);

        x.positions[6] = glm::vec4(-0.75, 0.75, 0.0, 0.0);
        x.positions[7] = glm::vec4(0.0, 0.75, 0.0, 0.0);
        x.positions[8] = glm::vec4(0.75, 0.75, 0.0, 0.0);
    });

    // 頂点バッファ
    let vertex_buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_size(std::mem::size_of::<glm::Vec2>() * 3)
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER),
    );
    vertex_buffer.map_as_slice_mut(3, |x: &mut [glm::Vec2]| {
        x[0] = glm::vec2(-0.15, -0.15);
        x[1] = glm::vec2(0.15, -0.15);
        x[2] = glm::vec2(0.0, 0.15);
    });

    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                let mut next_scan_buffer_view =
                    swap_chain.acquire_next_scan_buffer_view(None, None);

                command_buffer.begin();
                command_buffer.clear_color(
                    &mut next_scan_buffer_view,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    TextureArrayRange::new(),
                );
                command_buffer.set_render_targets(&[&next_scan_buffer_view], None);
                command_buffer.set_shader(&shader);
                command_buffer.set_constant_buffer(0, &positions_buffer);
                command_buffer.set_vertex_state(&vertex_state);
                command_buffer.set_vertex_buffer(0, &vertex_buffer);
                command_buffer.draw_instanced(
                    PrimitiveTopology::TriangleList,
                    3,
                    0, /*offset*/
                    9, /*instance_count*/
                    0, /*base_instnce*/
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
