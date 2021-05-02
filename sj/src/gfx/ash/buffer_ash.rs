use std::marker::PhantomData;
use ash::version::DeviceV1_0;
use ash::vk::DeviceSize;

use super::super::{Device, MemoryPool};
use super::super::buffer_api::{BufferInfo, IBufferImpl};

pub struct BufferImpl<'a>
{
	_device: &'a Device,
	_memory_pool: &'a MemoryPool<'a>,
	_buffer: ash::vk::Buffer,
	_offset: i64,
	_size: u64,
	_marker: PhantomData<&'a u32>,
}

impl<'a> IBufferImpl<'a> for BufferImpl<'a>
{
	fn new(device: &'a Device, _info: &BufferInfo, memory_pool: &'a MemoryPool, offset: i64, size: u64) -> Self
	{
		let device_impl = device.to_data().get_device();
		let memory_pool_impl = memory_pool.to_data().get_memory_pool();
		let buffer_create_info = ash::vk::BufferCreateInfo::builder()
			.size(32)
			.usage(ash::vk::BufferUsageFlags::INDEX_BUFFER)
			.sharing_mode(ash::vk::SharingMode::EXCLUSIVE);
		
		unsafe {
			let buffer = device_impl.create_buffer(&buffer_create_info, None).unwrap();
			device_impl.bind_buffer_memory(buffer, memory_pool_impl, offset as DeviceSize).expect("Unable to bind Buffer");
			
			Self
			{
				_device: device,
				_memory_pool: memory_pool,
				_offset: offset,
				_size: size,
				_buffer: buffer,
				_marker: PhantomData,
			}			
		}		
	}

	fn map<T>(&self) -> &mut T
	{
		let device_impl =  self._device.to_data().get_device();
		let device_memory = self._memory_pool.to_data().get_memory_pool();

		unsafe {
			let mapped_data = device_impl.map_memory(device_memory, self._offset as DeviceSize, self._size, ash::vk::MemoryMapFlags::empty()).unwrap();
			&mut *(mapped_data as *mut T)
		}
	}

	fn unmap(&self)
	{
		let device_impl =  self._device.to_data().get_device();
		let device_memory = self._memory_pool.to_data().get_memory_pool();
		unsafe {
			device_impl.unmap_memory(device_memory);
		}
	}
}

impl<'a> Drop for BufferImpl<'a>
{
	fn drop(&mut self){
		let device = self._device.to_data().get_device();
		unsafe {
			device.destroy_buffer(self._buffer, None);
		}
	}
}
