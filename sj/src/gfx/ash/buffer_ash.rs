use ash::version::DeviceV1_0;
use ash::vk::DeviceSize;
use std::marker::PhantomData;

use super::super::buffer_api::{BufferInfo, IBufferImpl};
use super::super::{BufferUsage, Device, MemoryPool};

pub struct BufferImpl<'a> {
    _device: &'a Device,
    _memory_pool: &'a MemoryPool<'a>,
    _buffer: ash::vk::Buffer,
    _offset: i64,
    _size: u64,
    _marker: PhantomData<&'a u32>,
}

impl<'a> BufferImpl<'a> {
    pub fn convert_usage(buffer_usage: BufferUsage) -> ash::vk::BufferUsageFlags {
        let mut result = ash::vk::BufferUsageFlags::empty();

        if buffer_usage.contains(BufferUsage::CONSTANT_BUFFER) {
            result |= ash::vk::BufferUsageFlags::UNIFORM_BUFFER;
        }
        if buffer_usage.contains(BufferUsage::UNORDERED_ACCESS_BUFFER) {
            result |= ash::vk::BufferUsageFlags::STORAGE_BUFFER
        }

        result
    }

    pub fn get_buffer(&self) -> ash::vk::Buffer {
        self._buffer
    }

    pub fn get_offset(&self) -> i64 {
        self._offset
    }
}

impl<'a> IBufferImpl<'a> for BufferImpl<'a> {
    fn new(
        device: &'a Device,
        info: &BufferInfo,
        memory_pool: &'a MemoryPool,
        offset: i64,
        size: u64,
    ) -> Self {
        let device_impl = device.to_data().get_device();
        let memory_pool_impl = memory_pool.to_data().get_memory_pool();
        let buffer_create_info = ash::vk::BufferCreateInfo::builder()
            .size(info.get_size())
            .usage(info.get_usage_as_ash())
            .sharing_mode(ash::vk::SharingMode::EXCLUSIVE);

        unsafe {
            let buffer = device_impl
                .create_buffer(&buffer_create_info, None)
                .unwrap();
            device_impl
                .bind_buffer_memory(buffer, memory_pool_impl, offset as DeviceSize)
                .expect("Unable to bind Buffer");

            Self {
                _device: device,
                _memory_pool: memory_pool,
                _offset: offset,
                _size: size,
                _buffer: buffer,
                _marker: PhantomData,
            }
        }
    }

    fn get_required_alignment(_device: &Device, _info: &BufferInfo) -> u64 {
        // let instance = device.to_data().get_instance();
        // let physical_device = device.to_data().get_physical_device();
        // unsafe  {
        // 	let physical_device_properties = instance.get_physical_device_properties(*physical_device);
        // 	physical_device_properties.limits.min_storage_buffer_offset_alignment
        // }
        256
    }

    fn map<T>(&self) -> &mut T {
        let device_impl = self._device.to_data().get_device();
        let device_memory = self._memory_pool.to_data().get_memory_pool();

        unsafe {
            let mapped_data = device_impl
                .map_memory(
                    device_memory,
                    self._offset as DeviceSize,
                    self._size,
                    ash::vk::MemoryMapFlags::empty(),
                )
                .unwrap();
            &mut *(mapped_data as *mut T)
        }
    }

    fn map_as_slice<T>(&self, count: usize) -> &[T] {
        let device_impl = self._device.to_data().get_device();
        let device_memory = self._memory_pool.to_data().get_memory_pool();

        unsafe {
            let mapped_data = device_impl
                .map_memory(
                    device_memory,
                    self._offset as DeviceSize,
                    self._size,
                    ash::vk::MemoryMapFlags::empty(),
                )
                .unwrap();
            std::slice::from_raw_parts(mapped_data as *mut T, count)
        }
    }

    fn map_as_slice_mut<T>(&self, count: usize) -> &mut [T] {
        let device_impl = self._device.to_data().get_device();
        let device_memory = self._memory_pool.to_data().get_memory_pool();

        unsafe {
            let mapped_data = device_impl
                .map_memory(
                    device_memory,
                    self._offset as DeviceSize,
                    self._size,
                    ash::vk::MemoryMapFlags::empty(),
                )
                .unwrap();
            std::slice::from_raw_parts_mut(mapped_data as *mut T, count)
        }
    }

    fn unmap(&self) {
        let device_impl = self._device.to_data().get_device();
        let device_memory = self._memory_pool.to_data().get_memory_pool();
        unsafe {
            device_impl.unmap_memory(device_memory);
        }
    }

    fn flush_mapped_range(&self, offset: i64, size: u64) {
        let mapped_memory_range = ash::vk::MappedMemoryRange::builder()
            .memory(self._memory_pool.to_data().get_memory_pool())
            .offset(offset as u64)
            .size(size)
            .build();
        unsafe {
            self._device
                .to_data()
                .get_device()
                .flush_mapped_memory_ranges(&[mapped_memory_range])
                .unwrap();
        }
    }

    fn invalidate_mapped_range(&self, offset: i64, size: u64) {
        unsafe {
            let mapped_memory_range = ash::vk::MappedMemoryRange::builder()
                .memory(self._memory_pool.to_data().get_memory_pool())
                .offset(offset as u64)
                .size(size)
                .build();
            self._device
                .to_data()
                .get_device()
                .invalidate_mapped_memory_ranges(&[mapped_memory_range])
                .unwrap();
        }
    }
}

impl<'a> Drop for BufferImpl<'a> {
    fn drop(&mut self) {
        let device = self._device.to_data().get_device();
        unsafe {
            device.destroy_buffer(self._buffer, None);
        }
    }
}

impl BufferInfo {
    pub fn get_usage_as_ash(&self) -> ash::vk::BufferUsageFlags {
        let mut result = ash::vk::BufferUsageFlags::empty();

        if self
            .get_buffer_usage()
            .contains(BufferUsage::UNORDERED_ACCESS_BUFFER)
        {
            result |= ash::vk::BufferUsageFlags::STORAGE_BUFFER;
        } else if self.get_buffer_usage().contains(BufferUsage::VERTEX_BUFFER) {
            result |= ash::vk::BufferUsageFlags::VERTEX_BUFFER;
        }

        result
    }
}
