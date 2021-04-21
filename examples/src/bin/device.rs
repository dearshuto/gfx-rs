fn main()
{
    let device_info = sj::gfx::DeviceInfo::new();
    let device = sj::gfx::Device::new(&device_info);
	
    let queue_info = sj::gfx::QueueInfo::new();
    let mut queue = sj::gfx::Queue::new(&device, &queue_info);

	let command_buffer_info = sj::gfx::CommandBufferInfo::new();
	let mut command_buffer = sj::gfx::CommandBuffer::new(&device, &command_buffer_info);

	let buffer_info = sj::gfx::BufferInfo::new();
	let _buffer = sj::gfx::Buffer::new(&device, &buffer_info);

	queue.execute(&command_buffer);
	queue.flush();
	queue.sync();
}
