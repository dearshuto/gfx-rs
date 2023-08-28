use crate::DeviceAsh;

pub struct FenceAsh {
    handle: ash::vk::Fence,

    device: ash::Device,
}

impl FenceAsh {
    pub fn new(device: &DeviceAsh) -> Self {
        let fence_create_info = ash::vk::FenceCreateInfo::builder()
            .flags(ash::vk::FenceCreateFlags::default())
            .build();
        let handle = unsafe { device.handle().create_fence(&fence_create_info, None) }.unwrap();
        Self {
            handle,
            device: device.handle(),
        }
    }

    pub fn begin(&self) {}

    pub fn end(&self) {}

    pub fn handle(&self) -> ash::vk::Fence {
        self.handle.clone()
    }
}

impl Drop for FenceAsh {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_fence(self.handle, None);
        }
    }
}
