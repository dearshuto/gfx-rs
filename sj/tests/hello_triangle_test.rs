use sj::gfx::{Buffer, BufferInfo, BufferTextureCopyRegion, CommandBuffer, CommandBufferInfo, Device, DeviceInfo, MemoryPool, MemoryPoolInfo, Queue, QueueInfo, Texture, TextureCopyRegion, TextureInfo};

#[test]
fn color_target_view_test()
{
	let device = Device::new(&DeviceInfo::new());
	let mut command_buffer = CommandBuffer::new(&device, &CommandBufferInfo::new());
	let mut queue = Queue::new(&device, &QueueInfo::new());

	let memory_pool = MemoryPool::new(&device, &MemoryPoolInfo::new());
	let texture = Texture::new(&device, &TextureInfo::new(), &memory_pool, 0, 0);

	let mut buffer = Buffer::new(&device, &BufferInfo::new(), &memory_pool, 0, 0);

	let copy_region = BufferTextureCopyRegion::new();
	command_buffer.copy_image_to_buffer(&mut buffer, &texture, &copy_region);
	queue.execute(&command_buffer);
	queue.sync();
}
