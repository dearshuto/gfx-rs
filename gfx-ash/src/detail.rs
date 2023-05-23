use ash::vk::DeviceSize;

use crate::DeviceAsh;

pub struct DeviceMemory {
    device: ash::Device,
    handle: ash::vk::DeviceMemory,
}

impl DeviceMemory {
    pub fn new(
        device: &DeviceAsh,
        size: usize,
        memory_requirement: Option<ash::vk::MemoryRequirements>,
    ) -> Self {
        // デバイスメモリ
        let instance = device.get_instance();
        let physical_device = device.get_physical_device();
        let properties = unsafe { instance.get_physical_device_memory_properties(physical_device) };
        let memory_type_index = properties.memory_types[..properties.memory_type_count as _]
            .iter()
            .enumerate()
            .find_map(|(index, memory_type)| {
                let memory_type_bits = if let Some(memory_requirement) = memory_requirement {
                    memory_requirement.memory_type_bits
                } else {
                    0
                };
                let memory_flags = ash::vk::MemoryPropertyFlags::DEVICE_LOCAL;
                let is_contains = ((memory_type_bits & (1 << index) as u32) != 0)
                    && (memory_type.property_flags & memory_flags == memory_flags);

                if is_contains {
                    Some(index)
                } else {
                    None
                }
            })
            .unwrap();

        let allocate_info = ash::vk::MemoryAllocateInfo::default()
            .allocation_size(size as DeviceSize)
            .memory_type_index(memory_type_index as u32);
        let device_memory =
            unsafe { device.get_device().allocate_memory(&allocate_info, None) }.unwrap();

        Self {
            device: device.get_device(),
            handle: device_memory,
        }
    }

    pub fn clone_device_memory_handle(&self) -> ash::vk::DeviceMemory {
        self.handle
    }
}

impl Drop for DeviceMemory {
    fn drop(&mut self) {
        unsafe { self.device.free_memory(self.handle, None) }
    }
}
