extern crate nalgebra_glm as glm;

use sjgfx::{
    api::IApi, TBufferBuilder, TColorTargetViewBuilder, TCommandBufferBuilder, TDeviceBuilder,
    TQueueBuilder, TSamplerBuilder, TSemaphoreBuilder, TShaderBuilder, TSwapChainBuilder,
    TTextureBuilder, TTextureViewBuilder, TVertexStateBuilder,
};
use sjgfx_interface::{
    AttributeFormat, DepthStencilStateInfo, IBuffer, ICommandBuffer, IDepthStencilView, IQueue,
    ISwapChain, ImageFormat, IndexFormat, PrimitiveTopology, VertexAttributeStateInfo,
    VertexBufferStateInfo,
};
use sjvi::{IDisplay, IInstance};

#[repr(C)]
struct Vertex2d {
    x: f32,
    y: f32,
    u: f32,
    v: f32,
}

#[allow(dead_code)]
#[repr(C)]
struct Vertex3d {
    x: f32,
    y: f32,
    z: f32,
}

#[allow(dead_code)]
#[repr(C)]
struct ConstantBuffer {
    pv: glm::Mat4x4,
}

fn main() {
    if cfg!(feature = "backend-glow") {
        run::<sjgfx::api::Glow>();
    } else {
        run::<sjgfx::api::Wgpu>();
    }
}

fn run<TApi: IApi>() {
    let mut instance = TApi::Instance::new();
    let id = instance.create_display();
    let display = instance.try_get_display(&id).unwrap();

    let mut device = TDeviceBuilder::<TApi>::new()
        .enable_debug_assertion()
        .build_with_surface(&display);
    let mut queue = TQueueBuilder::<TApi>::new().build(&mut device);
    let mut swap_chain = TSwapChainBuilder::<TApi>::new()
        .with_width(1280)
        .with_height(960)
        .build(&mut device);
    let mut semaphore = TSemaphoreBuilder::<TApi>::new().build(&device);
    let mut command_buffer = TCommandBufferBuilder::<TApi>::new().build(&device);
    let mut g_buffer_command_buffer = TCommandBufferBuilder::<TApi>::new().build(&device);

    let geometry_buffer_shader = TShaderBuilder::<TApi>::new()
        .set_vertex_shader_binary(include_bytes!(
            "../outputs/resources/shaders/geometry_output.vs.spv"
        ))
        .set_pixel_shader_binary(include_bytes!(
            "../outputs/resources/shaders/geometry_output.fs.spv"
        ))
        .build(&mut device);
    let shading_shader = TShaderBuilder::<TApi>::new()
        .set_vertex_shader_binary(include_bytes!(
            "../outputs/resources/shaders/deffered_shading.vs.spv"
        ))
        .set_pixel_shader_binary(include_bytes!(
            "../outputs/resources/shaders/deffered_shading.fs.spv"
        ))
        .build(&mut device);

    // G-Buffer
    let sampler = TSamplerBuilder::<TApi>::new().build(&mut device);
    let (albedo_buffer, normal_buffer, depth_buffer) = {
        let albedo_buffer = TTextureBuilder::<TApi>::new()
            .with_size(1280, 960)
            .with_format(ImageFormat::R8G8B8A8Unorm)
            .enable_color_buffer()
            .enable_sampler()
            .build(&mut device);

        let normal_buffer = TTextureBuilder::<TApi>::new()
            .with_size(1280, 960)
            .with_format(ImageFormat::R8G8B8A8Unorm)
            .enable_color_buffer()
            .enable_sampler()
            .build(&mut device);

        let depth_buffer = TTextureBuilder::<TApi>::new()
            .with_size(1280, 960)
            .with_format(ImageFormat::D32)
            .enable_depth_buffer()
            .enable_sampler()
            .build(&mut device);

        (albedo_buffer, normal_buffer, depth_buffer)
    };

    let albedo_view = TTextureViewBuilder::<TApi>::new()
        .with_format(ImageFormat::R8G8B8A8Unorm)
        .build(&device, &albedo_buffer);
    let normal_view = TTextureViewBuilder::<TApi>::new()
        .with_format(ImageFormat::R8G8B8A8Unorm)
        .build(&device, &normal_buffer);
    let depth_view = TTextureViewBuilder::<TApi>::new()
        .with_format(ImageFormat::D32)
        .build(&device, &depth_buffer);

    // レンダーターゲット
    let albedo_target_view = TColorTargetViewBuilder::<TApi>::new().build(&device, &albedo_buffer);
    let normal_target_view = TColorTargetViewBuilder::<TApi>::new().build(&device, &normal_buffer);
    let depth_stencil_view = TApi::DepthStencilView::new(
        &device,
        &DepthStencilStateInfo::new().set_depth_test_enabled(true),
        &depth_buffer,
    );

    // 矩形描画のデータ
    let rect_vertex_buffer = TBufferBuilder::<TApi>::new()
        .enable_vertex_buffer()
        .with_size(std::mem::size_of::<Vertex2d>() * 6)
        .build(&mut device);
    rect_vertex_buffer.map_as_slice_mut(|x| {
        x[0] = Vertex2d {
            x: -1.0,
            y: 1.0,
            u: 0.0,
            v: 0.0,
        };
        x[1] = Vertex2d {
            x: -1.0,
            y: -1.0,
            u: 0.0,
            v: 1.0,
        };
        x[2] = Vertex2d {
            x: 1.0,
            y: -1.0,
            u: 1.0,
            v: 1.0,
        };
        x[3] = Vertex2d {
            x: -1.0,
            y: 1.0,
            u: 0.0,
            v: 0.0,
        };
        x[4] = Vertex2d {
            x: 1.0,
            y: -1.0,
            u: 1.0,
            v: 1.0,
        };
        x[5] = Vertex2d {
            x: 1.0,
            y: 1.0,
            u: 1.0,
            v: 0.0,
        };
    });
    let rect_vertex_state = TVertexStateBuilder::<TApi>::new()
        .set_vertex_attribute_states(
            [
                VertexAttributeStateInfo::new()
                    .set_buffer_index(0)
                    .set_format(AttributeFormat::Float32_32)
                    .set_offset(0)
                    .set_slot(0),
                VertexAttributeStateInfo::new()
                    .set_buffer_index(0)
                    .set_format(AttributeFormat::Float32_32)
                    .set_offset((std::mem::size_of::<f32>() * 2) as i64)
                    .set_slot(1),
            ]
            .into_iter(),
        )
        .set_vertex_buffer_states(
            [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<Vertex2d>() as i64)]
                .into_iter(),
        )
        .build(&device);

    // 3D モデルのデータ
    let obj_data = sjgfx_examples::load_obj(
        &mut device,
        &include_str!("../resources/models/standard_bunny/stanford_bunny_res4.obj"),
    );
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
    let vertex_state = TVertexStateBuilder::<TApi>::new()
        .set_vertex_attribute_states(vertex_attribute_state_info_array.into_iter())
        .set_vertex_buffer_states(vertex_buffer_state_info_array.into_iter())
        .build(&device);

    // 定数バッファ
    let constant_buffer = TBufferBuilder::<TApi>::new()
        .enable_constant_buffer()
        .with_size(std::mem::size_of::<ConstantBuffer>())
        .build(&mut device);
    constant_buffer.map_mut(|x: &mut ConstantBuffer| {
        let position = glm::vec3(0.0, 1.5, -1.0);
        let at = glm::vec3(0.0, 0.0, -0.5);
        let up = glm::vec3(0.0, 0.0, -1.0);
        let view_matrix: glm::Mat4x4 = glm::look_at(&position, &at, &up);
        let fov = std::f32::consts::PI / 4.0;
        let projection_matrix: glm::Mat4x4 = glm::perspective_fov(fov, 640.0, 480.0, 0.1, 100.0);

        x.pv = projection_matrix * view_matrix;
    });

    // G-Buffer を出力するコマンドを生成
    g_buffer_command_buffer.begin();
    g_buffer_command_buffer.set_render_targets(
        &[&albedo_target_view, &normal_target_view],
        Some(&depth_stencil_view),
    );
    g_buffer_command_buffer.set_shader(&geometry_buffer_shader);
    g_buffer_command_buffer.set_constant_buffer(0, &constant_buffer);
    g_buffer_command_buffer.set_vertex_state(&vertex_state);
    g_buffer_command_buffer.set_vertex_buffer(0, &obj_data.vertex_buffer);
    g_buffer_command_buffer.draw_indexed(
        PrimitiveTopology::TriangleList,
        IndexFormat::Uint32,
        &obj_data.index_buffer,
        obj_data.index_count,
        0, /*base_vertex*/
    );
    g_buffer_command_buffer.end();

    while instance.try_update() {
        let display = instance.try_get_display(&id).unwrap();
        if display.is_redraw_requested() {
            let next_scan_buffer_view =
                swap_chain.acquire_next_scan_buffer_view(Some(&mut semaphore), None);

            command_buffer.begin();
            command_buffer.set_render_targets(&[&next_scan_buffer_view], None);
            command_buffer.set_shader(&shading_shader);
            command_buffer.set_texture(0, &albedo_view);
            command_buffer.set_texture(1, &normal_view);
            command_buffer.set_texture(2, &depth_view);
            command_buffer.set_sampler(3, &sampler);
            command_buffer.set_vertex_state(&rect_vertex_state);
            command_buffer.set_vertex_buffer(0, &rect_vertex_buffer);
            command_buffer.draw(PrimitiveTopology::TriangleList, 6, 0);
            command_buffer.end();

            queue.execute(&g_buffer_command_buffer);
            queue.execute(&command_buffer);
            queue.present(&mut swap_chain);
            queue.flush();
            queue.sync();

            display.listen(&mut swap_chain);
        }
    }
}
