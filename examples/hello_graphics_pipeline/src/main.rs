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

    let viewport_state_info = [sj::gfx::ViewportStateInfo::new()
        .set_origin_x(0.0)
        .set_origin_y(0.0)
        .set_width(640.0)
        .set_height(480.0)];
    let scissor_state_info = [sj::gfx::ScissorStateInfo::new()
        .set_origin_x(0)
        .set_origin_y(0)
        .set_width(640)
        .set_height(480)];
    let viewport_scissor_state_info = sj::gfx::ViewportScissorStateInfo::new()
        .set_viewport_state_info_array(&viewport_state_info)
        .set_scissor_state_info_array(&scissor_state_info);
    let viewport_scissor_state =
        sj::gfx::ViewportScissorState::new(&device, &viewport_scissor_state_info);

    let vertex_attribute_state_info_array = [sj::gfx::VertexAttributeStateInfo::new()
        .set_slot(0)
        .set_format(sj::gfx::AttributeFormat::Float32_32)
        .set_offset(0)
        .set_buffer_index(0)];
    let vertex_buffer_state_info_array = [sj::gfx::VertexBufferStateInfo::new().set_stride(16)];
    let vertex_state_info = sj::gfx::VertexStateInfo::new()
        .set_attribute_state_info_array(&vertex_attribute_state_info_array)
        .set_buffer_state_info_array(&vertex_buffer_state_info_array);
    let rasterizer_state_info = sj::gfx::RasterizerStateInfo::new();
    let depth_stencil_state_info = sj::gfx::DepthStencilStateInfo::new();
    let blend_target_state_info_array = [sj::gfx::BlendTargetStateInfo::new()];
    let blend_state_info =
        sj::gfx::BlendStateInfo::new().set_target_state_info(&blend_target_state_info_array);

    let graphics_pipeline_create_info = sj::gfx::GraphicsPipelineInfo::new()
        .set_vertex_state_info(&vertex_state_info)
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

    let texture_memory_pool_info = sj::gfx::MemoryPoolInfo::new()
        .set_memory_pool_property(
            sj::gfx::MemoryPoolProperty::CPU_INVISIBLE | sj::gfx::MemoryPoolProperty::GPU_CACHED,
        )
        .set_size(1024 * 1024 * 1024);
    let texture_memory_pool = sj::gfx::MemoryPool::new(&device, &texture_memory_pool_info);
    let texture_info = sj::gfx::TextureInfo::new()
        .set_width(640)
        .set_height(480)
        .set_depth(1)
        .set_gpu_access_flags(sj::gfx::GpuAccess::COLOR_BUFFER)
        .set_image_format(sj::gfx::ImageFormat::R8G8B8A8Unorm);
    let texture = sj::gfx::Texture::new(&device, &texture_info, &texture_memory_pool, 0, 0);

    let color_target_view_info = sj::gfx::ColorTargetViewInfo::new(&texture)
        .set_image_format(sj::gfx::ImageFormat::R8G8B8A8Unorm);
    let color_target_view = sj::gfx::ColorTargetView::new(&device, &color_target_view_info);

    let buffer_info = sj::gfx::BufferInfo::new()
        .set_gpu_access_flags(sj::gfx::GpuAccess::VERTEX_BUFFER)
        .set_size(128);
    let vertex_buffer = sj::gfx::Buffer::new(&device, &buffer_info, &memory_pool, 0, 128);

    let a = vertex_buffer.map_as_slice_mut::<f32>(16);
    a[0] = 10.0;
    a[1] = 10.0;
    vertex_buffer.flush_mapped_range(0, 0x40);
    vertex_buffer.unmap();

    let value = vertex_buffer.map::<f32>();
    println!("{}", value);
    vertex_buffer.unmap();

    let command_buffer_info = sj::gfx::CommandBufferInfo::new();
    let mut command_buffer = sj::gfx::CommandBuffer::new(&device, &command_buffer_info);

    let queue_info = sj::gfx::QueueInfo::new();
    let mut queue = sj::gfx::Queue::new(&device, &queue_info);

    command_buffer.begin();
    {
        //		command_buffer.clear_color(&mut &mut color_target_view, 0.0, 0.0, 0.0, 0.0);
        command_buffer.set_render_targets(&[&color_target_view], None);
        command_buffer.set_viewport_scissor_state(&viewport_scissor_state);
        command_buffer.set_pipeline(&pipeline);
        command_buffer.set_vertex_buffer(0, &sj::gfx::GpuAddress::new(&vertex_buffer));

        let vertex_count = 3;
        let vertex_offset = 0;
        command_buffer.draw(
            sj::gfx::PrimitiveTopology::TriangleList,
            vertex_count,
            vertex_offset,
        );
    }
    command_buffer.end();

    for _index in 0..1 {
        queue.execute(&command_buffer);
        queue.flush();
        queue.sync();
    }
}
