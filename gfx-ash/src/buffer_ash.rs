use sjgfx_interface::BufferInfo;

use crate::{DeviceAsh, MemoryPoolAsh};

pub struct BufferAsh {
    handle: ash::vk::Buffer,

    device: ash::Device,

    device_memory: ash::vk::DeviceMemory,
}

impl BufferAsh {
    pub fn new(
        device: &DeviceAsh,
        info: BufferInfo,
        memory_pool: &MemoryPoolAsh,
        offset: isize,
        _size: usize,
    ) -> Self {
        // TODO: バッファーの使用方法はちゃんと選ぶ
        let buffer_create_info = ash::vk::BufferCreateInfo::builder()
            .usage(ash::vk::BufferUsageFlags::STORAGE_BUFFER)
            .size(info.get_size() as u64)
            .sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            .build();
        let buffer = unsafe { device.handle().create_buffer(&buffer_create_info, None) }.unwrap();

        // デバイスメモリとのひも付け
        unsafe {
            device
                .handle()
                .bind_buffer_memory(buffer, memory_pool.handle(), offset as u64)
        }
        .unwrap();

        Self {
            handle: buffer,
            device: device.handle(),
            device_memory: memory_pool.handle(),
        }
    }

    pub fn map_mut<T, F: Fn(&mut T)>(&self, func: F) {
        let raw_ptr = unsafe {
            self.device.map_memory(
                self.device_memory,
                0,
                std::mem::size_of::<T>() as u64,
                ash::vk::MemoryMapFlags::empty(),
            )
        }
        .unwrap();
        let data = raw_ptr as *mut T;
        unsafe { func(&mut *data) };

        unsafe { self.device.unmap_memory(self.device_memory) };
    }

    pub fn handle(&self) -> ash::vk::Buffer {
        self.handle
    }
}

impl Drop for BufferAsh {
    fn drop(&mut self) {
        unsafe { self.device.destroy_buffer(self.handle, None) };
    }
}
