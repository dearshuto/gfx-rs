use crate::DeviceAsh;

pub struct MemoryPoolAsh {
    handle: ash::vk::DeviceMemory,

    device: ash::Device,
}

impl MemoryPoolAsh {
    pub fn new(device: &DeviceAsh) -> Self {
        let properties = unsafe {
            crate::SHARED_INSTANCE
                .get()
                .unwrap()
                .instance
                .get_physical_device_memory_properties(device.get_physical_device_handle())
        };
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

        let memory_allocate_info = ash::vk::MemoryAllocateInfo::builder()
            .allocation_size(1024)
            .memory_type_index(memory_type_index as u32)
            .build();
        let handle =
            unsafe { device.handle().allocate_memory(&memory_allocate_info, None) }.unwrap();
        Self {
            handle,
            device: device.handle(),
        }
    }

    pub fn handle(&self) -> ash::vk::DeviceMemory {
        self.handle
    }
}

impl Drop for MemoryPoolAsh {
    fn drop(&mut self) {
        unsafe {
            self.device.free_memory(self.handle, None);
        }
    }
}
