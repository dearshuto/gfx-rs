use ash::version::{DeviceV1_0, InstanceV1_0};
use std::marker::PhantomData;

use super::super::memory_pool_api::{IMemoryPoolImpl, MemoryPoolInfo};
use super::super::Device;

pub struct MemoryPoolImpl<'a> {
    _device: &'a Device,
    _memory_pool: ash::vk::DeviceMemory,
    _marker: PhantomData<&'a u32>,
}

impl<'a> MemoryPoolImpl<'a> {
    pub fn get_memory_pool(&self) -> ash::vk::DeviceMemory {
        self._memory_pool
    }
}

impl<'a> IMemoryPoolImpl<'a> for MemoryPoolImpl<'a> {
    fn new(device: &'a Device, info: &MemoryPoolInfo) -> Self {
        let instance = device.to_data().get_instance();
        let physical_device = device.to_data().get_physical_device();
        let device_impl = device.to_data().get_device();

        unsafe {
            let memory_type_index = instance
                .get_physical_device_memory_properties(*physical_device)
                .memory_types
                .iter()
                .enumerate()
                .find_map(|(index, memory_type)| {
                    let memory_flags = ash::vk::MemoryPropertyFlags::DEVICE_LOCAL
                        | ash::vk::MemoryPropertyFlags::HOST_VISIBLE;
                    let is_contains = memory_type.property_flags.contains(memory_flags);

                    if is_contains {
                        Some(index)
                    } else {
                        None
                    }
                })
                .unwrap();

            let allocate_info = ash::vk::MemoryAllocateInfo::builder()
                .allocation_size(info.get_size())
                .memory_type_index(memory_type_index as u32);

            let memory_pool = device_impl.allocate_memory(&allocate_info, None).unwrap();
            Self {
                _device: device,
                _memory_pool: memory_pool,
                _marker: PhantomData,
            }
        }
    }
}

impl<'a> Drop for MemoryPoolImpl<'a> {
    fn drop(&mut self) {
        unsafe {
            let device_impl = self._device.to_data().get_device();
            device_impl.free_memory(self._memory_pool, None);
        }
    }
}
