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
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;

#[repr(C)]
#[derive(Default, Clone)]
struct Vertex {
    #[allow(dead_code)]
    pub x: f32,

    #[allow(dead_code)]
    pub y: f32,

    #[allow(dead_code)]
    pub z: f32,
}

#[repr(C)]
#[derive(Clone)]
struct ConstantBuffer {
    pv: glm::Mat4x4,
}

impl Default for ConstantBuffer {
    fn default() -> Self {
        Self {
            pv: glm::Mat4x4::identity(),
        }
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

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
        &include_str!("resources/shaders/hello_3d.vs"),
        sjgfx_util::ShaderStage::Vertex,
    );
    let pixel_shader_binary = compiler.create_binary(
        &include_str!("resources/shaders/hello_3d.fs"),
        sjgfx_util::ShaderStage::Pixel,
    );
    let shader = ShaderWgpu::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(&vertex_shader_binary)
            .set_pixel_shader_binary(&pixel_shader_binary),
    );

    let attribute_state_info_array = [
        VertexAttributeStateInfo::new()
            .set_buffer_index(0)
            .set_format(AttributeFormat::Float32_32_32)
            .set_offset(0)
            .set_slot(0),
        VertexAttributeStateInfo::new()
            .set_buffer_index(0)
            .set_format(AttributeFormat::Float32_32_32)
            .set_offset(0) // 位置をそのまま法線として使ってる
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

    let position = glm::vec3(1.5, 1.0, 3.0);
    let at = glm::vec3(0.0, 0.0, 0.0);
    let up = glm::vec3(0.0, 1.0, 0.0);
    let view_matrix: glm::Mat4x4 = glm::look_at(&position, &at, &up);
    let fov = std::f32::consts::PI / 4.0;
    let projection_matrix: glm::Mat4x4 = glm::perspective_fov(fov, 640.0, 480.0, 0.1, 100.0);
    let pv = (projection_matrix * view_matrix).transpose();
    let constant_buffer_data = ConstantBuffer { pv };
    let data = unsafe {
        std::slice::from_raw_parts(
            (&constant_buffer_data as *const ConstantBuffer) as *const u8,
            std::mem::size_of::<ConstantBuffer>(),
        )
    };

    let constant_buffer = BufferWgpu::new_init(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER)
            .set_size(std::mem::size_of::<ConstantBuffer>()),
        &data,
    );

    let mut swap_chain = SwapChainWgpu::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );

    let mut t: f32 = 0.0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        t += 1.0;
        window.request_redraw();
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_size),
                ..
            } => {
                // TODO
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                let mut color_target_view = swap_chain.acquire_next_scan_buffer_view(None, None);

                command_buffer.begin();
                command_buffer.clear_color(
                    &mut color_target_view,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    TextureArrayRange::new(),
                );
                command_buffer.set_shader(&shader);
                command_buffer.set_render_targets(&[&color_target_view], None);
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

                constant_buffer.map_mut(|x: &mut ConstantBuffer| {
                    let position =
                        glm::vec3(5.5 * t.to_radians().cos(), 1.0 * t.to_radians().sin(), 3.0);
                    let at = glm::vec3(0.0, 0.0, 0.0);
                    let up = glm::vec3(0.0, 1.0, 0.0);
                    let view_matrix: glm::Mat4x4 = glm::look_at(&position, &at, &up);
                    let fov = std::f32::consts::PI / 4.0;
                    let projection_matrix: glm::Mat4x4 =
                        glm::perspective_fov(fov, 640.0, 480.0, 0.1, 100.0);
                    let pv = (projection_matrix * view_matrix).transpose();
                    x.pv = pv;
                });
            }
            _ => {}
        }
    });
}
