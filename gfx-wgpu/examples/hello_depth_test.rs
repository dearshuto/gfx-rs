use sjgfx_interface::{
    AttributeFormat, BufferInfo, CommandBufferInfo, DepthStencilStateInfo, DeviceInfo, GpuAccess,
    IBuffer, IColorTargetView, ICommandBuffer, IDepthStencilView, IDevice, IQueue, IShader,
    ISwapChain, ITexture, IVertexState, ImageFormat, IndexFormat, PrimitiveTopology, QueueInfo,
    ShaderInfo, SwapChainInfo, TextureInfo, VertexAttributeStateInfo, VertexBufferStateInfo,
    VertexStateInfo,
};
use sjgfx_wgpu::{
    BufferWgpu, ColorTargetViewWgpu, CommandBufferWgpu, DepthStencilViewWgpu, DeviceWgpu,
    QueueWgpu, ShaderWgpu, SwapChainWgpu, TextureWgpu, VertexStateWgpu,
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
    run::<
        DeviceWgpu,
        QueueWgpu,
        CommandBufferWgpu,
        ShaderWgpu,
        BufferWgpu,
        VertexStateWgpu,
        ColorTargetViewWgpu,
        DepthStencilViewWgpu,
        SwapChainWgpu,
        TextureWgpu,
    >();
}

fn run<
    TDevice,
    TQueue,
    TCommandBuffer,
    TShader,
    TBuffer,
    TVertexState,
    TColorTargetView,
    TDepthStencilView,
    TSwapChain,
    TTexture,
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
        ColorTargetViewType = TColorTargetView,
        ShaderType = TShader,
        BufferType = TBuffer,
        DepthStencilViewType = TDepthStencilView,
        VertexStateType = TVertexState,
    >,
    TShader: IShader<DeviceType = TDevice>,
    TBuffer: IBuffer<DeviceType = TDevice>,
    TVertexState: IVertexState<DeviceType = TDevice>,
    TColorTargetView: IColorTargetView<DeviceType = TDevice>,
    TDepthStencilView: IDepthStencilView<DeviceType = TDevice, TextureType = TTexture>,
    TSwapChain: ISwapChain<DeviceType = TDevice, ColorTargetViewType = TColorTargetView>,
    TTexture: ITexture<DeviceType = TDevice>,
{
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut device = TDevice::new_with_surface(&DeviceInfo::new(), &window, &event_loop);
    let mut queue = TQueue::new(&device, &QueueInfo::new());
    let mut command_buffer = TCommandBuffer::new(&device, &CommandBufferInfo::new());

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
    let shader = TShader::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(vertex_shader_binary.as_binary_u8())
            .set_pixel_shader_binary(pixel_shader_binary.as_binary_u8()),
    );

    let attribute_state_info_array = [VertexAttributeStateInfo::new()
        .set_buffer_index(0)
        .set_format(AttributeFormat::Float32_32_32)
        .set_offset(0)
        .set_slot(0)];
    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<Vertex>() as i64)];
    let vertex_state = TVertexState::new(
        &device,
        &VertexStateInfo::new()
            .set_attribute_state_info_array(attribute_state_info_array.into_iter())
            .set_buffer_state_info_array(vertex_buffer_state_info_array),
    );

    let vertex_buffer = TBuffer::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(std::mem::size_of::<Vertex>() * 6),
    );
    vertex_buffer.map_as_slice_mut(|buffer| {
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

    let index_buffer = TBuffer::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::INDEX_BUFFER)
            .set_size(std::mem::size_of::<u32>() * 6),
    );
    index_buffer.map_as_slice_mut(|buffer| {
        buffer[0] = 0;
        buffer[1] = 1;
        buffer[2] = 2;

        buffer[3] = 3;
        buffer[4] = 4;
        buffer[5] = 5;
    });

    // 深度バッファ
    let texture = TTexture::new(
        &device,
        &TextureInfo::new()
            .set_width(1280)
            .set_height(960)
            .set_image_format(ImageFormat::D32)
            .set_gpu_access_flags(GpuAccess::DEPTH_STENCIL),
    );
    let depth_stencil_view =
        TDepthStencilView::new(&device, &DepthStencilStateInfo::new(), &texture);

    let mut swap_chain = TSwapChain::new(
        &mut device,
        &SwapChainInfo::new().with_width(1280).with_height(960),
    );

    let mut should_close = false;
    while !should_close {
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    let color_target_view = swap_chain.acquire_next_scan_buffer_view(None, None);

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
