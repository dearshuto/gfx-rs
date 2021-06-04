struct Data {
    pub value: f32,
}

fn main() {
    let device_info = sj::gfx::DeviceInfo::new();
    let device = sj::gfx::Device::new(&device_info);

    let vertex_shader_source =
        include_bytes!("../resources/shaders/hello_graphics_pipeline_vs.spv");
    let pixel_shader_source = include_bytes!("../resources/shaders/hello_graphics_pipeline_fs.spv");
    let shader_info = sj::gfx::ShaderInfo::new()
        .set_vertex_shader_binary(vertex_shader_source)
        .set_pixel_shader_binary(pixel_shader_source);
    let shader = sj::gfx::Shader::new(&device, &shader_info);

    let rasterizer_state_info = sj::gfx::RasterizerStateInfo::new();
    let depth_stencil_state_info = sj::gfx::DepthStencilStateInfo::new();
    let blend_state_info = sj::gfx::BlendStateInfo::new();

    let graphics_pipeline_create_info = sj::gfx::GraphicsPipelineInfo::new()
        .set_rasterizer_state(&rasterizer_state_info)
        .set_depth_stencil_state(&depth_stencil_state_info)
        .set_blend_stae(&blend_state_info)
        .set_shader(&shader);
    let pipeline = sj::gfx::Pipeline::new_as_graphics(&device, &graphics_pipeline_create_info);

    let memory_pool_info = sj::gfx::MemoryPoolInfo::new()
        .set_size(1024)
        .set_memory_pool_property(
            sj::gfx::MemoryPoolProperty::CPU_CACHED | sj::gfx::MemoryPoolProperty::GPU_CACHED,
        );
    let memory_pool = sj::gfx::MemoryPool::new(&device, &memory_pool_info);

    let texture_memory_pool = sj::gfx::MemoryPool::new(&device, &sj::gfx::MemoryPoolInfo::new());
    let texture_info = sj::gfx::TextureInfo::new();
    let texture = sj::gfx::Texture::new(&device, &texture_info, &texture_memory_pool, 0, 0);

    let color_target_view_info = sj::gfx::ColorTargetViewInfo::new(&texture);
    let mut color_target_view = sj::gfx::ColorTargetView::new(&device, &color_target_view_info);

    let buffer_info = sj::gfx::BufferInfo::new()
        .set_buffer_usage(sj::gfx::BufferUsage::VERTEX_BUFFER)
        .set_size(64);
    let vertex_buffer = sj::gfx::Buffer::new(&device, &buffer_info, &memory_pool, 0, 64);

    let mut a = vertex_buffer.map::<Data>();
    a.value = 10.0;
    vertex_buffer.unmap();

    let d = vertex_buffer.map::<Data>();
    println!("{}", d.value);
    vertex_buffer.unmap();

    let command_buffer_info = sj::gfx::CommandBufferInfo::new();
    let mut command_buffer = sj::gfx::CommandBuffer::new(&device, &command_buffer_info);

    let queue_info = sj::gfx::QueueInfo::new();
    let mut queue = sj::gfx::Queue::new(&device, &queue_info);

    command_buffer.clear_color(&mut &mut color_target_view, 0.0, 0.0, 0.0, 0.0);
    command_buffer.set_render_targets(&[&color_target_view], None);
    command_buffer.set_pipeline(&pipeline);
    command_buffer.set_vertex_buffer(0, &sj::gfx::GpuAddress::new(&vertex_buffer));

    let vertex_count = 3;
    let vertex_offset = 0;
    command_buffer.draw(
        sj::gfx::PrimitiveTopology::TriangleList,
        vertex_count,
        vertex_offset,
    );

    for _index in 0..1 {
        queue.execute(&command_buffer);
        queue.flush();
        queue.sync();
    }
}
