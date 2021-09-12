struct IntData {
    pub value: u32,
}

fn main() {
    let device_info = sj::gfx::DeviceInfo::new();
    let device = sj::gfx::Device::new(&device_info);

    let memory_pool = sj::gfx::MemoryPool::new(
        &device,
        &sj::gfx::MemoryPoolInfo::new()
            .set_size(1024)
            .set_memory_pool_property(
                sj::gfx::MemoryPoolProperty::CPU_CACHED | sj::gfx::MemoryPoolProperty::GPU_CACHED,
            ),
    );

    let source = include_bytes!("../resources/shaders/hello_compute.spv");
    let shader_info = sj::gfx::ShaderInfo::new().set_shader_binary(source);
    let shader = sj::gfx::Shader::new(&device, &shader_info);

    let pipeline_info = sj::gfx::ComputePipelineInfo::new().set_shader(&shader);
    let pipeline = sj::gfx::Pipeline::new_as_compute(&device, pipeline_info);

    let buffer_info = sj::gfx::BufferInfo::new()
        .set_size(64)
        .set_gpu_access_flags(sj::gfx::GpuAccess::UNORDERED_ACCESS_BUFFER);
    let required_alignment = sj::gfx::Buffer::get_required_alignment(&device, &buffer_info);
    let buffer = sj::gfx::Buffer::new(
        &device,
        &buffer_info,
        &memory_pool,
        required_alignment as i64,
        buffer_info.get_size(),
    );

    let command_buffer_info = sj::gfx::CommandBufferInfo::new();
    let mut command_buffer = sj::gfx::CommandBuffer::new(&device, &command_buffer_info);

    let queue_info = sj::gfx::QueueInfo::new();
    let mut queue = sj::gfx::Queue::new(&device, &queue_info);

    let gpu_address = sj::gfx::GpuAddress::new(&buffer);
    command_buffer.begin();
    command_buffer.set_pipeline(&pipeline);
    command_buffer.set_unordered_access_buffer(0, sj::gfx::ShaderStage::Compute, &gpu_address, 64);
    command_buffer.dispatch(1, 1, 1);
    command_buffer.end();

    queue.execute(&command_buffer);
    queue.flush();
    queue.sync();

    // buffer.map();
    // buffer.invalidate_mapped_range(0, 64);
    // buffer.read::<IntData>(|mapped_data| println!("{}", mapped_data.value) );
    // buffer.unmap();
}
