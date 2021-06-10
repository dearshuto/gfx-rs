struct Data {
    pub value: i32,
}

fn main() {
    let device_info = sj::gfx::DeviceInfo::new();
    let device = sj::gfx::Device::new(&device_info);

    let _rasterizer_state =
        sj::gfx::RasterizerState::new(&device, sj::gfx::RasterizerStateInfo::new());
    let _depth_stencil_state =
        sj::gfx::DepthStencilState::new(&device, &sj::gfx::DepthStencilStateInfo::new());
    let _blend_state = sj::gfx::BlendState::new(&device, &sj::gfx::BlendStateInfo::new());

    let memory_pool_info = sj::gfx::MemoryPoolInfo::new()
        .set_size(1024)
        .set_memory_pool_property(
            sj::gfx::MemoryPoolProperty::CPU_CACHED | sj::gfx::MemoryPoolProperty::GPU_CACHED,
        );
    let memory_pool = sj::gfx::MemoryPool::new(&device, &memory_pool_info);

    let buffer_info = sj::gfx::BufferInfo::new()
        .set_size(64)
        .set_gpu_access_flags(sj::gfx::GpuAccess::CONSTANT_BUFFER);
    let buffer = sj::gfx::Buffer::new(&device, &buffer_info, &memory_pool, 0, 64);

    let texture_memory_pool = sj::gfx::MemoryPool::new(
        &device,
        &sj::gfx::MemoryPoolInfo::new()
            .set_size(1024 * 1024)
            .set_memory_pool_property(sj::gfx::MemoryPoolProperty::CPU_INVISIBLE),
    );
    let texture_info = sj::gfx::TextureInfo::new()
        .set_width(128)
        .set_height(128)
        .set_gpu_access_flags(sj::gfx::GpuAccess::TEXTURE);
    let texture_required_size = sj::gfx::Texture::calculate_required_size(&device, &texture_info);
    let texture_memory_pool_offset = 0;
    let _texture = sj::gfx::Texture::new(
        &device,
        &texture_info,
        &texture_memory_pool,
        texture_memory_pool_offset,
        texture_required_size,
    );

    let mut a = buffer.map::<Data>();
    a.value = 10;
    buffer.unmap();

    let d = buffer.map::<Data>();
    println!("{}", d.value);
    buffer.unmap();

    let command_buffer_info = sj::gfx::CommandBufferInfo::new();
    let command_buffer = sj::gfx::CommandBuffer::new(&device, &command_buffer_info);

    let queue_info = sj::gfx::QueueInfo::new();
    let mut queue = sj::gfx::Queue::new(&device, &queue_info);

    for _i in 0..2 {
        //		command_buffer.set_pipeline(&pipeline);
        queue.execute(&command_buffer);

        queue.flush();
        queue.sync();
    }
}
