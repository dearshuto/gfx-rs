struct IntData {
    pub value: u32,
}

fn main() {
    let device_info = sj::gfx::DeviceInfo::new();
    let device = sj::gfx::Device::new(&device_info);

    let memory_pool = sj::gfx::MemoryPool::new(&device, &sj::gfx::MemoryPoolInfo::new());

    let source = include_bytes!("../resources/shaders/hello_compute.spv");
    let shader_info = sj::gfx::ShaderInfo::new().set_shader_binary(source);
    let shader = sj::gfx::Shader::new(&device, &shader_info);

    let pipeline_info = sj::gfx::ComputePipelineInfo::new().set_shader(&shader);
    let pipeline = sj::gfx::Pipeline::new_as_compute(&device, pipeline_info);

    let buffer = sj::gfx::Buffer::new(&device, &sj::gfx::BufferInfo::new(), &memory_pool, 0, 16);

	let command_buffer_info = sj::gfx::CommandBufferInfo::new();
    let mut command_buffer = sj::gfx::CommandBuffer::new(&device, &command_buffer_info);
	
    let queue_info = sj::gfx::QueueInfo::new();
    let mut queue = sj::gfx::Queue::new(&device, &queue_info);

	command_buffer.begin();
    command_buffer.set_pipeline(&pipeline);
    command_buffer.set_buffer(&buffer);
    command_buffer.dispatch(1, 1, 1);
	command_buffer.end();
	
    queue.execute(&command_buffer);
    queue.flush();
    queue.sync();

    let mapped_data = buffer.map::<IntData>();
    println!("{}", mapped_data.value);
    buffer.unmap();
}
