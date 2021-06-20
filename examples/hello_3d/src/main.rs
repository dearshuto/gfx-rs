extern crate nalgebra_glm as glm;

#[repr(C)]
struct ConstantBuffer {
    pv: glm::Mat4x4,
}

fn main() {
    let position = glm::vec3(1.5, 1.0, 3.0);
    let at = glm::vec3(0.0, 0.0, 0.0);
    let up = glm::vec3(0.0, 1.0, 0.0);
    let view_matrix: glm::Mat4x4 = glm::look_at(&position, &at, &up);
    let fov = std::f32::consts::PI / 4.0;
    let projection_matrix: glm::Mat4x4 = glm::perspective_fov(fov, 640.0, 480.0, 0.1, 100.0);

    let device_info = sj::gfx::DeviceInfo::new();
    let device = sj::gfx::Device::new(&device_info);

    let mut display = sj::vi::create_display();
    let mut layer = sj::vi::create_layer(&mut display);

    let mut swap_shain_info = sj::gfx::SwapChainInfo::new(&mut layer);
    let mut swap_chain = sj::gfx::SwapChain::new(&device, &mut swap_shain_info);
    let (_scan_buffers, scan_buffer_views) = swap_chain.get_scan_buffers_and_views();

    let memory_pool = sj::gfx::MemoryPool::new(
        &device,
        &sj::gfx::MemoryPoolInfo::new()
            .set_size(1024)
            .set_memory_pool_property(
                sj::gfx::MemoryPoolProperty::CPU_CACHED | sj::gfx::MemoryPoolProperty::GPU_CACHED,
            ),
    );

    let vertex_shader_source = include_bytes!("../resources/shaders/hello_3d_vs.spv");
    let pixel_shader_source = include_bytes!("../resources/shaders/hello_3d_fs.spv");
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
        .set_format(sj::gfx::AttributeFormat::Float32_32_32)
        .set_offset(0)
        .set_buffer_index(0)];
    let vertex_buffer_state_info_array = [
        sj::gfx::VertexBufferStateInfo::new().set_stride((std::mem::size_of::<f32>() * 3) as i64)
    ];
    let vertex_state_info = sj::gfx::VertexStateInfo::new()
        .set_attribute_state_info_array(&vertex_attribute_state_info_array)
        .set_buffer_state_info_array(&vertex_buffer_state_info_array);
    let rasterizer_state_info = sj::gfx::RasterizerStateInfo::new();
    let depth_stencil_state_info = sj::gfx::DepthStencilStateInfo::new()
        .set_depth_test_enabled(true)
        .set_depth_write_enabled(true);
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

    let constant_buffer_info = sj::gfx::BufferInfo::new()
        .set_size(std::mem::size_of::<ConstantBuffer>() as u64)
        .set_gpu_access_flags(sj::gfx::GpuAccess::CONSTANT_BUFFER);
    let constant_buffer = sj::gfx::Buffer::new(
        &device,
        &constant_buffer_info,
        &memory_pool,
        0,
        constant_buffer_info.get_size(),
    );
    {
        let mut mapped_data = constant_buffer.map::<ConstantBuffer>();
        mapped_data.pv = projection_matrix * view_matrix;
    }
    constant_buffer.flush_mapped_range(0, std::mem::size_of::<ConstantBuffer>() as u64);
    constant_buffer.unmap();

    let vertex_buffer_memory_pool = sj::gfx::MemoryPool::new(
        &device,
        &sj::gfx::MemoryPoolInfo::new()
            .set_size(1024)
            .set_memory_pool_property(
                sj::gfx::MemoryPoolProperty::CPU_CACHED | sj::gfx::MemoryPoolProperty::GPU_CACHED,
            ),
    );
    let vertex_buffer = sj::gfx::Buffer::new(
        &device,
        &sj::gfx::BufferInfo::new()
            .set_size((std::mem::size_of::<f32>() * 3 * 6) as u64)
            .set_gpu_access_flags(sj::gfx::GpuAccess::VERTEX_BUFFER),
        &vertex_buffer_memory_pool,
        0,
        (std::mem::size_of::<f32>() * 27) as u64,
    );
    {
        let mut vertex_data = vertex_buffer.map_as_slice_mut::<f32>(18);
        vertex_data[0] = 0.0;
        vertex_data[1] = 1.0;
        vertex_data[2] = 0.0;

        vertex_data[3] = 0.0;
        vertex_data[4] = 0.0;
        vertex_data[5] = 1.0;

        vertex_data[6] = 1.0;
        vertex_data[7] = 0.0;
        vertex_data[8] = 0.0;

        vertex_data[9] = 0.0;
        vertex_data[10] = 1.0;
        vertex_data[11] = 0.0;

        vertex_data[12] = -1.0;
        vertex_data[13] = 0.0;
        vertex_data[14] = 0.0;

        vertex_data[15] = 0.0;
        vertex_data[16] = 0.0;
        vertex_data[17] = 1.0;
    }
    vertex_buffer.flush_mapped_range(0, 0x40 /*(std::mem::size_of::<f32>() * 9) as u64*/);
    vertex_buffer.unmap();

    // 震度ステンシルビュー
    let depth_texture_memory_pool = sj::gfx::MemoryPool::new(
        &device,
        &sj::gfx::MemoryPoolInfo::new()
            .set_size(32 * 640 * 480)
            .set_memory_pool_property(
                sj::gfx::MemoryPoolProperty::CPU_INVISIBLE
                    | sj::gfx::MemoryPoolProperty::GPU_CACHED,
            ),
    );
    let depth_texture = sj::gfx::Texture::new(
        &device,
        &sj::gfx::TextureInfo::new()
            .set_width(640)
            .set_height(480)
            .set_image_format(sj::gfx::ImageFormat::D32)
            .set_gpu_access_flags(sj::gfx::GpuAccess::DEPTH_STENCIL | sj::gfx::GpuAccess::TEXTURE),
        &depth_texture_memory_pool,
        0,
        1024,
    );
    let mut depth_stencil_view = sj::gfx::DepthStencilView::new(
        &device,
        &sj::gfx::DepthStencilViewInfo::new(&depth_texture),
    );

    let mut semaphore = sj::gfx::Semaphore::new(&device, &sj::gfx::SemaphoreInfo::new());

    let command_buffer_info = sj::gfx::CommandBufferInfo::new();
    let mut command_buffers = [
        sj::gfx::CommandBuffer::new(&device, &command_buffer_info),
        sj::gfx::CommandBuffer::new(&device, &command_buffer_info),
    ];

    let queue_info = sj::gfx::QueueInfo::new();
    let mut queue = sj::gfx::Queue::new(&device, &queue_info);

    for index in 0..command_buffers.len() {
        let command_buffer = &mut command_buffers[index];

        command_buffer.begin();
        command_buffer.clear_color(&mut scan_buffer_views[index], 0.25, 0.25, 0.4, 1.0, None);
        command_buffer.clear_depth_stencil(
            &mut depth_stencil_view,
            1.0,
            0,
            &sj::gfx::DepthStencilClearMode::Depth,
            None,
        );
        command_buffer
            .flush_memory(sj::gfx::GpuAccess::COLOR_BUFFER | sj::gfx::GpuAccess::DEPTH_STENCIL);

        command_buffer.set_render_targets(&[&scan_buffer_views[index]], Some(&depth_stencil_view));
        command_buffer.set_viewport_scissor_state(&&viewport_scissor_state);
        command_buffer.set_pipeline(&pipeline);
        command_buffer.set_constant_buffer(
            0,
            sj::gfx::ShaderStage::Vertex,
            &sj::gfx::GpuAddress::new(&constant_buffer),
            std::mem::size_of::<ConstantBuffer>(),
        );
        command_buffer.set_vertex_buffer(0, &sj::gfx::GpuAddress::new(&vertex_buffer));
        command_buffer.draw(sj::gfx::PrimitiveTopology::TriangleList, 6, 0);
        command_buffer.flush_memory(sj::gfx::GpuAccess::COLOR_BUFFER);
        command_buffer.end();
    }

    for _i in 0..500 {
        let index = swap_chain.acquire_next_scan_buffer_index(Some(&mut semaphore), None);
        queue.sync_semaphore(&mut semaphore);

        let command_buffer = &command_buffers[index as usize];
        queue.execute(&command_buffer);
        queue.present(&mut swap_chain, 1);
        queue.flush();
        queue.sync();
        std::thread::sleep(std::time::Duration::from_millis(32));
    }
}
