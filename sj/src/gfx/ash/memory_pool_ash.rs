use std::marker::PhantomData;
use ash::version::DeviceV1_0;

use super::super::Device;
use super::super::memory_pool_api::{IMemoryPoolImpl, MemoryPoolInfo};

pub struct MemoryPoolImpl<'a>
{
	_device: &'a Device,
	_memory_pool: ash::vk::DeviceMemory,
	_marker: PhantomData<&'a u32>,
}

impl<'a> MemoryPoolImpl<'a>
{
	pub fn get_memory_pool(&self) -> ash::vk::DeviceMemory
	{
		self._memory_pool
	}
}

impl<'a> IMemoryPoolImpl<'a> for MemoryPoolImpl<'a>
{	
	fn new(device: &'a Device, info: &MemoryPoolInfo) -> Self
	{
		let allocate_info = ash::vk::MemoryAllocateInfo::builder()
			.allocation_size(info.get_size())
			.memory_type_index(1); // TODO
		
		unsafe {
			let device_impl = device.to_data().get_device();
			let memory_pool = device_impl.allocate_memory(&allocate_info, None).unwrap();

			Self{
				_device: device,
				_memory_pool: memory_pool,
				_marker: PhantomData,
			}
		}
	}
}

impl<'a> Drop for MemoryPoolImpl<'a>
{
	fn drop(&mut self)
	{
		unsafe{
			let device_impl = self._device.to_data().get_device();
			device_impl.free_memory(self._memory_pool, None);
		}
	}
}
