extern crate nalgebra_glm as glm;
use sjgfx::{BufferBuilder, CommandBufferBuilder, DeviceBuilder, QueueBuilder, SwapChainBuilder};

use sjgfx_interface::{IndexFormat, PrimitiveTopology};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

#[repr(C)]
struct ConstantBuffer {
    pv: glm::Mat4x4,
    time: f32,
}

fn main() {
    let particle_count = 16;
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut device = DeviceBuilder::new().build_with_surface(&window, &event_loop);
    let mut queue = QueueBuilder::new().build(&device);
    let mut swap_chain = SwapChainBuilder::new().build(&mut device);
    let mut command_buffer = CommandBufferBuilder::new().build(&device);
    let mut compute_command_buffer = CommandBufferBuilder::new().build(&device);
    let simulation_buffer = BufferBuilder::new()
        .with_size(64)
        .enable_vertex_buffer()
        .build(&device);

    // 定数バッファ
    let constant_buffer = BufferBuilder::new()
        .with_size(std::mem::size_of::<ConstantBuffer>())
        .enable_constant_buffer()
        .build(&device);
    constant_buffer.map_mut(|x: &mut ConstantBuffer| {
        let position = glm::vec3(5.0, 1.5, 5.0);
        let at = glm::vec3(0.0, 0.0, -0.5);
        let up = glm::vec3(0.0, 0.0, -1.0);
        let view_matrix: glm::Mat4x4 = glm::look_at(&position, &at, &up);
        let fov = std::f32::consts::PI / 4.0;
        let projection_matrix: glm::Mat4x4 = glm::perspective_fov(fov, 640.0, 480.0, 0.1, 100.0);

        x.pv = projection_matrix * view_matrix;
    });

    // 頂点バッファ、インデクスバッファ
    let obj_data = sjgfx_examples::load_obj(
        &device,
        &include_str!("../resources/models/sphere/sphere.obj"),
    );

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    compute_command_buffer.begin();
                    // compute_command_buffer.set_shader(shader);
                    compute_command_buffer.set_unordered_access_buffer(0, &simulation_buffer);
                    compute_command_buffer.dispatch(1, 1, 1);
                    compute_command_buffer.end();

                    let next_scan_buffer_view =
                        swap_chain.acquire_next_scan_buffer_view(None, None);

                    command_buffer.begin();
                    command_buffer.set_render_targets([next_scan_buffer_view].into_iter(), None);
                    // command_buffer.set_shader(&shader);
                    command_buffer.set_constant_buffer(0, &constant_buffer);
                    // command_buffer.set_vertex_state(&vertex_state);
                    command_buffer.set_vertex_buffer(0, &obj_data.vertex_buffer);
                    command_buffer.set_vertex_buffer(1, &simulation_buffer);
                    command_buffer.draw_indexed_instanced(
                        PrimitiveTopology::TriangleList,
                        IndexFormat::Uint32,
                        &obj_data.index_buffer,
                        obj_data.index_count,
                        0,              /*base_vertex*/
                        particle_count, /*instance_count*/
                        0,              /*base_instance*/
                    );
                    command_buffer.end();

                    queue.execute(&compute_command_buffer);
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
        std::thread::sleep(std::time::Duration::from_millis(32));
    }
}
