extern crate nalgebra_glm as glm;

use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DepthStencilStateInfo, DeviceInfo, GpuAccess,
    IBuffer, IColorTargetView, ICommandBuffer, IDepthStencilView, IDevice, IQueue, IShader,
    ISwapChain, ITexture, IVertexState, ImageFormat, IndexFormat, PrimitiveTopology, QueueInfo,
    ShaderInfo, SwapChainInfo, TextureArrayRange, TextureInfo, VertexAttributeStateInfo,
    VertexBufferStateInfo, VertexStateInfo,
};
use sjgfx_wgpu::{
    BufferWgpu, ColorTargetViewWgpu, CommandBufferWgpu, DepthStencilViewWgpu, DeviceWgpu,
    QueueWgpu, ShaderWgpu, SwapChainWgpu, TextureWgpu, VertexStateWgpu,
};
use winit::event_loop::EventLoop;

#[repr(C)]
struct ConstantBuffer {
    pv: glm::Mat4x4,
    time: f32,
    _padding0: f32,
    _padding1: f32,
    _padding2: f32,
}

fn main() {
    run::<
        DeviceWgpu,
        QueueWgpu,
        CommandBufferWgpu,
        SwapChainWgpu,
        ColorTargetViewWgpu,
        DepthStencilViewWgpu,
        ShaderWgpu,
        BufferWgpu,
        TextureWgpu,
        VertexStateWgpu,
    >();
}

fn run<
    TDevice,
    TQueue,
    TCommandBuffer,
    TSwapChain,
    TColorTargetView,
    TDepthStencilView,
    TShader,
    TBuffer,
    TTexture,
    TVertexState,
>()
where
    TDevice: IDevice,
    TQueue: IQueue<
        DeviceType = TDevice,
        CommandBufferType = TCommandBuffer,
        SwapChainType = TSwapChain,
    >,
    TCommandBuffer: ICommandBuffer<
        DeviceType = TDevice,
        ShaderType = TShader,
        BufferType = TBuffer,
        ColorTargetViewType = TColorTargetView,
        DepthStencilViewType = TDepthStencilView,
        VertexStateType = TVertexState,
    >,
    TSwapChain: ISwapChain<DeviceType = TDevice, ColorTargetViewType = TColorTargetView>,
    TColorTargetView: IColorTargetView<DeviceType = TDevice>,
    TDepthStencilView: IDepthStencilView<DeviceType = TDevice, TextureType = TTexture>,
    TShader: IShader<DeviceType = TDevice>,
    TBuffer: IBuffer<DeviceType = TDevice>,
    TTexture: ITexture<DeviceType = TDevice>,
    TVertexState: IVertexState<DeviceType = TDevice>,
{
    let event_loop = EventLoop::new();
    let mut display = sjvi::winit::create_display(event_loop);

    let mut device = TDevice::new_with_surface(
        &DeviceInfo::new(),
        &display.window,
        display.event_loop.as_ref().unwrap(),
    );
    let mut queue = TQueue::new(&device, &QueueInfo::new());
    let mut command_buffer = TCommandBuffer::new(&device, &CommandBufferInfo::new());
    let mut swap_chain = TSwapChain::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );

    // シェーダ
    let vertex_shader_binary = include_bytes!("../outputs/resources/shaders/wave.vs.spv");
    let pixel_shader_binary = include_bytes!("../outputs/resources/shaders/wave.fs.spv");
    let shader = TShader::new(
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
    let vertex_state = TVertexState::new(&device, &vertex_state_info);

    // 定数バッファ
    let constant_buffer = TBuffer::new(
        &mut device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER)
            .set_size(std::mem::size_of::<ConstantBuffer>()),
    );
    constant_buffer.map_mut(|x: &mut ConstantBuffer| {
        let position = glm::vec3(1.6, 1.5, -2.0);
        let at = glm::vec3(0.0, 0.0, 0.0);
        let up = glm::vec3(0.0, 1.0, 0.0);
        let view_matrix: glm::Mat4x4 = glm::look_at(&position, &at, &up);
        let fov = std::f32::consts::PI / 4.0;
        let projection_matrix: glm::Mat4x4 = glm::perspective_fov(fov, 640.0, 480.0, 0.1, 100.0);

        x.pv = projection_matrix * view_matrix;
        x.time = 0.0;
    });

    // 頂点バッファ、インデクスバッファ
    let obj_data = sjgfx_examples::load_obj(
        &mut device,
        &include_str!("../resources/models/plane/plane.obj"),
    );

    // 深度バッファ
    let depth_buffer = TTexture::new(
        &mut device,
        &TextureInfo::new()
            .set_width(1280)
            .set_height(960)
            .set_gpu_access_flags(GpuAccess::DEPTH_STENCIL)
            .set_image_format(ImageFormat::D32),
    );
    let depth_stencil_view =
        TDepthStencilView::new(&device, &DepthStencilStateInfo::new(), &depth_buffer);

    while !display.should_close() {
        display.update(|| {
            constant_buffer.map_mut(|x: &mut ConstantBuffer| {
                x.time += 0.05;
            });

            // queue.sync_semaphore(&mut semaphore);
            let mut next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view(None, None);

            command_buffer.begin();
            command_buffer.clear_color(
                &mut next_scan_buffer_view,
                0.0,
                0.0,
                0.1,
                1.0,
                TextureArrayRange::new(),
            );
            command_buffer.set_render_targets(&[&next_scan_buffer_view], Some(&depth_stencil_view));
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
        });

        display.listen(&mut swap_chain);
    }
}
