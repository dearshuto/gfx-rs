extern crate nalgebra_glm as glm;

use sjgfx::api::IApi;
use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DepthStencilStateInfo, DeviceInfo, GpuAccess,
    IBuffer, ICommandBuffer, IDepthStencilView, IDevice, IQueue, ISemaphore, IShader, ISwapChain,
    ITexture, IVertexState, ImageFormat, IndexFormat, PrimitiveTopology, QueueInfo, SemaphoreInfo,
    ShaderInfo, SwapChainInfo, TextureArrayRange, TextureInfo, VertexAttributeStateInfo,
    VertexBufferStateInfo, VertexStateInfo,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

#[repr(C)]
struct ConstantBuffer {
    pv: glm::Mat4x4,
}

fn main() {
    if cfg!(feature = "backend-ash") {
        run::<sjgfx::api::Ash>();
    } else if cfg!(feature = "backend-wgpu") {
        run::<sjgfx::api::Wgpu>();
    } else {
        println!("");
    }
}

fn run<TApi: IApi>() {
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1280, 960))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut device = TApi::Device::new_with_surface(&DeviceInfo::new(), &window, &event_loop);
    let mut queue = TApi::Queue::new(&device, &QueueInfo::new());
    let mut command_buffer = TApi::CommandBuffer::new(&device, &CommandBufferInfo::new());
    let mut swap_chain = TApi::SwapChain::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );

    // シェーダ
    let vertex_shader_binary = include_bytes!("../outputs/resources/shaders/armadillo.vs.spv");
    let pixel_shader_binary = include_bytes!("../outputs/resources/shaders/armadillo.fs.spv");
    let shader = TApi::Shader::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(vertex_shader_binary)
            .set_pixel_shader_binary(pixel_shader_binary),
    );

    // 頂点ステート
    let vertex_attribute_state_info_array = [
        VertexAttributeStateInfo::new()
            .set_slot(0)
            .set_format(AttributeFormat::Float32_32_32)
            .set_offset(0)
            .set_buffer_index(0),
        VertexAttributeStateInfo::new()
            .set_slot(1)
            .set_format(AttributeFormat::Float32_32_32)
            .set_offset((std::mem::size_of::<f32>() * 3) as i64)
            .set_buffer_index(0),
    ];
    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride((std::mem::size_of::<f32>() * 6) as i64)];
    let vertex_state_info = VertexStateInfo::new()
        .set_attribute_state_info_array(vertex_attribute_state_info_array.into_iter())
        .set_buffer_state_info_array(vertex_buffer_state_info_array.into_iter());
    let vertex_state = TApi::VertexState::new(&device, &vertex_state_info);

    // 定数バッファ
    let constant_buffer = TApi::Buffer::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER)
            .set_size(std::mem::size_of::<ConstantBuffer>()),
    );
    constant_buffer.map_mut(|x: &mut ConstantBuffer| {
        let position = glm::vec3(0.0, 1.5, -1.0);
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
        &include_str!("../resources/models/standard_bunny/stanford_bunny_res4.obj"),
    );

    // 深度バッファ
    let depth_buffer = TApi::Texture::new(
        &device,
        &TextureInfo::new()
            .set_width(1280)
            .set_height(960)
            .set_gpu_access_flags(GpuAccess::DEPTH_STENCIL)
            .set_image_format(ImageFormat::D32),
    );
    let depth_stencil_view =
        TApi::DepthStencilView::new(&device, &DepthStencilStateInfo::new(), &depth_buffer);

    let mut semaphore = TApi::Semaphore::new(&device, &SemaphoreInfo::new());

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    // queue.sync_semaphore(&mut semaphore);

                    let mut next_scan_buffer_view =
                        swap_chain.acquire_next_scan_buffer_view(Some(&mut semaphore), None);

                    command_buffer.begin();
                    command_buffer.clear_color(
                        &mut next_scan_buffer_view,
                        0.0,
                        0.0,
                        0.1,
                        1.0,
                        TextureArrayRange::new(),
                    );
                    command_buffer.set_render_targets(
                        [next_scan_buffer_view].into_iter(),
                        Some(&depth_stencil_view),
                    );
                    command_buffer.set_shader(&shader);
                    command_buffer.set_constant_buffer(0, &constant_buffer);
                    command_buffer.set_vertex_state(&vertex_state);
                    command_buffer.set_vertex_buffer(0, &obj_data.vertex_buffer);
                    command_buffer.draw_indexed(
                        PrimitiveTopology::TriangleList,
                        IndexFormat::Uint32,
                        &obj_data.index_buffer,
                        obj_data.index_count,
                        0, /*base_vertex*/
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
        std::thread::sleep(std::time::Duration::from_millis(32));
    }
}
