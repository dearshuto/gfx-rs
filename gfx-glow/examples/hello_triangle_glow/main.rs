use std::mem::size_of;

use glow::HasContext;
use sjgfx_glow::VertexStateGlow;
use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IBuffer, ICommandBuffer,
    IQueue, ISwapChain, IVertexState, PrimitiveTopology, QueueInfo, SwapChainInfo,
    VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    sjgfx_glow::initialize();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Sample")
        .with_visible(true)
        .with_inner_size(PhysicalSize::new(640, 480))
        .build(&event_loop)
        .unwrap();
    let mut device = sjgfx_glow::DeviceGlow::new_from_handle(&DeviceInfo::new(), &window);
    let mut swap_chain = sjgfx_glow::SwapChainGlow::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );
    let mut command_buffer = sjgfx_glow::CommandBufferGlow::new(&device, &CommandBufferInfo::new());
    let mut queue = sjgfx_glow::QueueGlow::new(&mut device, &QueueInfo::new());

    let vertex_shader_source = include_str!("resources/hello_triangle.vs");
    let pixel_shader_source = include_str!("resources/hello_triangle.fs");
    let shader = sjgfx_glow::ShaderGlow::new(
        &mut device,
        &sjgfx_interface::ShaderInfo::new()
            .set_vertex_shader_source(vertex_shader_source)
            .set_pixel_shader_source(pixel_shader_source),
    );

    let vertex_state = VertexStateGlow::new(
        &device,
        &VertexStateInfo::new()
            .set_attribute_state_info_array([VertexAttributeStateInfo::new()
                .set_buffer_index(0)
                .set_slot(0)
                .set_format(AttributeFormat::Float32_32)
                .set_offset(0)])
            .set_buffer_state_info_array([
                VertexBufferStateInfo::new().set_stride((size_of::<f32>() * 2) as i64)
            ]),
    );

    let vertex_buffer = sjgfx_glow::BufferGlow::new(
        &mut device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(256),
    );
    let error = unsafe { device.clone_context().get_error() };
    if error != glow::NO_ERROR {
        println!("ERROR: {}", error);
    }

    vertex_buffer.map_as_slice_mut(|data: &mut [f32]| {
        data[0] = -0.5;
        data[1] = -0.5;
        data[2] = 0.5;
        data[3] = -0.5;
        data[4] = 0.0;
        data[5] = 0.6;
    });
    let error = unsafe { device.clone_context().get_error() };
    if error != glow::NO_ERROR {
        println!("MAP ERROR: {}", error);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_size),
                ..
            } => {
                // TODO
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                // スキャンバッファの取得
                let scan_buffer = swap_chain.acquire_next_scan_buffer_view(None, None);

                command_buffer.begin();
                command_buffer.set_render_targets(&[scan_buffer], None);
                command_buffer.set_shader(&shader);
                command_buffer.set_vertex_state(&vertex_state);
                command_buffer.set_vertex_buffer(0, &vertex_buffer);
                command_buffer.draw(PrimitiveTopology::TriangleList, 3, 0);
                command_buffer.end();

                queue.execute(&command_buffer);
                queue.present(&mut swap_chain);
                queue.flush();
            }
            _ => {}
        }
    });
}
