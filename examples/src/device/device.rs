struct Data{
	pub value: i32,
}

fn main()
{	
    let device_info = sj::gfx::DeviceInfo::new();
    let device = sj::gfx::Device::new(&device_info);
    let queue_info = sj::gfx::QueueInfo::new();
    let mut queue = sj::gfx::Queue::new(&device, &queue_info);

	let shader_info = sj::gfx::ShaderInfo::new();
	let shader = sj::gfx::Shader::new(&device, &shader_info);

	let pipeline_info = sj::gfx::PipelineInfo::new().set_shader(&shader);
	let pipeline = sj::gfx::Pipeline::new(&device, &pipeline_info);
	
	let memory_pool_info = sj::gfx::MemoryPoolInfo::new();
	let memory_pool = sj::gfx::MemoryPool::new(&device, &memory_pool_info);

	let buffer_info = sj::gfx::BufferInfo::new();
	let buffer = sj::gfx::Buffer::new(&device, &buffer_info, &memory_pool, 0, 64);

	let mut a = buffer.map::<Data>();
	a.value = 10;
	buffer.unmap();

	let d = buffer.map::<Data>();
	println!("{}", d.value);
	buffer.unmap();
		
	let command_buffer_info = sj::gfx::CommandBufferInfo::new();
	let mut command_buffer = sj::gfx::CommandBuffer::new(&device, &command_buffer_info);

	for _i in 0..2
	{
		command_buffer.set_pipeline(&pipeline);
		queue.execute(&command_buffer);
		
		queue.flush();
		queue.sync();
	}
}
