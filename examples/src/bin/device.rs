fn main()
{
    let device_info = sj::gfx::DeviceInfo::new();
    let mut device = sj::gfx::Device::new(&device_info);
	
    let queue_info = sj::gfx::QueueInfo::new();
    let mut queue = sj::gfx::Queue::new(&mut device, &queue_info);

	let command_buffer_info = sj::gfx::CommandBufferInfo::new();
	let command_buffer = sj::gfx::CommandBuffer::new(&mut device, &command_buffer_info);

	let buffer_info = sj::gfx::BufferInfo::new();
	let buffer = sj::gfx::Buffer::new(&mut device, &buffer_info);
}
