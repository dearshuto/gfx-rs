use ash::vk::DeviceSize;
use sjgfx_interface::{BufferInfo, GpuAccess, IBuffer};

use crate::DeviceAsh;

pub struct BufferAsh {
    device: ash::Device,
    buffer: ash::vk::Buffer,
    device_memory: ash::vk::DeviceMemory,
    size: usize,
}

impl BufferAsh {
    pub fn new(device: &DeviceAsh, info: &BufferInfo) -> Self {
        let buffer_usage = Self::convert_gpu_access_to_ash(&info.get_gpu_access_flags());
        let buffer_create_info = ash::vk::BufferCreateInfo::builder()
            .size(info.get_size() as ash::vk::DeviceSize)
            .usage(buffer_usage)
            .sharing_mode(ash::vk::SharingMode::EXCLUSIVE);

        let buffer =
            unsafe { device.get_device().create_buffer(&buffer_create_info, None) }.unwrap();

        // デバイスメモリ
        let instance = device.get_instance();
        let physical_device = device.get_physical_device();
        let properties = unsafe { instance.get_physical_device_memory_properties(physical_device) };
        let memory_type_index = properties
            .memory_types
            .iter()
            .enumerate()
            .find_map(|(index, memory_type)| {
                let memory_flags = ash::vk::MemoryPropertyFlags::HOST_VISIBLE;
                let is_contains = memory_type.property_flags.contains(memory_flags);

                if is_contains {
                    Some(index)
                } else {
                    None
                }
            })
            .unwrap();

        let allocate_info = ash::vk::MemoryAllocateInfo::builder()
            .allocation_size(info.get_size() as DeviceSize)
            .memory_type_index(memory_type_index as u32);
        let device_memory =
            unsafe { device.get_device().allocate_memory(&allocate_info, None) }.unwrap();

        // メモリのバインド
        unsafe {
            device
                .get_device()
                .bind_buffer_memory(buffer, device_memory, 0)
        }
        .unwrap();

        Self {
            device: device.get_device(),
            buffer,
            device_memory,
            size: info.get_size(),
        }
    }

    pub fn map<T, F: Fn(&T)>(&self, func: F) {
        let mapped_data_raw = unsafe {
            self.device.map_memory(
                self.device_memory,
                0, /*offset*/
                self.size as ash::vk::DeviceSize,
                ash::vk::MemoryMapFlags::empty(),
            )
        }
        .unwrap();
        let mapped_data = mapped_data_raw as *mut T;

        unsafe { func(&*mapped_data) }
        unsafe { self.device.unmap_memory(self.device_memory) }
    }

    pub fn map_mut<T, F: Fn(&mut T)>(&self, func: F) {
        let mapped_data_raw = unsafe {
            self.device.map_memory(
                self.device_memory,
                0, /*offset*/
                self.size as ash::vk::DeviceSize,
                ash::vk::MemoryMapFlags::empty(),
            )
        }
        .unwrap();
        let mapped_data = mapped_data_raw as *mut T;

        unsafe { func(&mut *mapped_data) }
        unsafe { self.device.unmap_memory(self.device_memory) }
    }

    pub fn map_as_slice<T, F: Fn(&[T])>(&self, size: usize, func: F) {
        let mapped_data_raw = unsafe {
            self.device.map_memory(
                self.device_memory,
                0, /*offset*/
                self.size as ash::vk::DeviceSize,
                ash::vk::MemoryMapFlags::empty(),
            )
        }
        .unwrap();
        let mapped_data = mapped_data_raw as *mut T;
        let mapped_slice_data = unsafe { std::slice::from_raw_parts::<T>(mapped_data, size) };

        func(mapped_slice_data);
        unsafe { self.device.unmap_memory(self.device_memory) }
    }

    // pub fn map_as_slice_mut<T, F: Fn(&mut [T])>(&self, size: usize, func: F) {
    //     let _result = self.buffer.slice(..).map_async(wgpu::MapMode::Write);

    //     self.device.get_device().poll(wgpu::Maintain::Wait);

    //     let ptr = self
    //         .buffer
    //         .slice(..)
    //         .get_mapped_range_mut()
    //         .as_mut_ptr()
    //         .cast::<T>();
    //     let slice = unsafe { std::slice::from_raw_parts_mut::<T>(ptr, size) };
    //     func(slice);
    //     self.buffer.unmap();
    // }

    pub fn flush_mapped_range(&self, offset: isize, size: usize) {
        let mapped_memory_range = ash::vk::MappedMemoryRange::builder()
            .memory(self.device_memory)
            .offset(offset as ash::vk::DeviceSize)
            .size(size as ash::vk::DeviceSize)
            .build();
        unsafe {
            self.device
                .flush_mapped_memory_ranges(&[mapped_memory_range])
        }
        .unwrap();
    }

    pub fn invalidate_mapped_range(&self, offset: isize, size: usize) {
        unsafe {
            self.device.map_memory(
                self.device_memory,
                offset as ash::vk::DeviceSize,
                size as ash::vk::DeviceSize,
                ash::vk::MemoryMapFlags::empty(),
            )
        }
        .unwrap();

        let mapped_memory_range = ash::vk::MappedMemoryRange::builder()
            .memory(self.device_memory)
            .offset(offset as ash::vk::DeviceSize)
            .size(size as ash::vk::DeviceSize)
            .build();
        unsafe {
            self.device
                .invalidate_mapped_memory_ranges(&[mapped_memory_range])
        }
        .unwrap();

        unsafe { self.device.unmap_memory(self.device_memory) }
    }

    pub fn get_buffer(&self) -> ash::vk::Buffer {
        self.buffer
    }

    fn convert_gpu_access_to_ash(gpu_access: &GpuAccess) -> ash::vk::BufferUsageFlags {
        let mut result = ash::vk::BufferUsageFlags::empty();

        if gpu_access.contains(GpuAccess::UNORDERED_ACCESS_BUFFER) {
            result |= ash::vk::BufferUsageFlags::STORAGE_BUFFER;
        }
        if gpu_access.contains(GpuAccess::VERTEX_BUFFER) {
            result |= ash::vk::BufferUsageFlags::VERTEX_BUFFER;
        }
        if gpu_access.contains(GpuAccess::INDEX_BUFFER) {
            result |= ash::vk::BufferUsageFlags::INDEX_BUFFER;
        }
        if gpu_access.contains(GpuAccess::CONSTANT_BUFFER) {
            result |= ash::vk::BufferUsageFlags::UNIFORM_BUFFER;
        }
        if gpu_access.contains(GpuAccess::READ) {
            result |= ash::vk::BufferUsageFlags::TRANSFER_SRC;
        }
        if gpu_access.contains(GpuAccess::WRITE) {
            result |= ash::vk::BufferUsageFlags::TRANSFER_DST;
        }

        result
    }
}

impl Drop for BufferAsh {
    fn drop(&mut self) {
        unsafe { self.device.free_memory(self.device_memory, None) }
        unsafe { self.device.destroy_buffer(self.buffer, None) }
    }
}

impl IBuffer for BufferAsh {
    type DeviceType = DeviceAsh;

    fn new(device: &Self::DeviceType, info: &BufferInfo) -> Self {
        Self::new(device, info)
    }

    fn map<T, F: Fn(&T)>(&self, func: F) {
        self.map(func);
    }

    fn map_mut<T, F: Fn(&mut T)>(&self, func: F) {
        self.map_mut(func);
    }

    fn map_as_slice<T, F: Fn(&[T])>(&self, func: F) {
        self.map_as_slice(64, func);
    }

    fn map_as_slice_mut<T, F: Fn(&mut [T])>(&self, _func: F) {
        todo!()
    }

    fn flush_mapped_range(&self, offset: isize, size: usize) {
        self.flush_mapped_range(offset, size);
    }

    fn invalidate_mapped_range(&self, offset: isize, size: usize) {
        self.invalidate_mapped_range(offset, size);
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{BufferInfo, DeviceInfo, GpuAccess};

    use crate::{BufferAsh, DeviceAsh};

    #[test]
    fn new() {
        let device = DeviceAsh::new(&DeviceInfo::new());
        let _buffer = BufferAsh::new(
            &device,
            &BufferInfo::new()
                .set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER)
                .set_size(128),
        );
        let _buffer = BufferAsh::new(
            &device,
            &BufferInfo::new()
                .set_gpu_access_flags(GpuAccess::UNORDERED_ACCESS_BUFFER)
                .set_size(128),
        );
        let _buffer = BufferAsh::new(
            &device,
            &BufferInfo::new()
                .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
                .set_size(128),
        );
        let _buffer = BufferAsh::new(
            &device,
            &BufferInfo::new()
                .set_gpu_access_flags(GpuAccess::INDEX_BUFFER)
                .set_size(128),
        );
    }

    #[test]
    fn map() {
        let device = DeviceAsh::new(&DeviceInfo::new());
        let buffer = BufferAsh::new(
            &device,
            &BufferInfo::new()
                .set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER)
                .set_size(std::mem::size_of::<i32>()),
        );
        buffer.map(|_x: &i32| {});
        buffer.map_mut(|x: &mut i32| *x = 10);
    }
}