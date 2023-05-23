use sjgfx_interface::IFence;

use crate::DeviceAsh;

pub struct FenceAsh {
    device: ash::Device,
    fence: ash::vk::Fence,
}

impl FenceAsh {
    pub fn new(device: &DeviceAsh) -> Self {
        let device = device.get_device();

        let fence_create_info =
            ash::vk::FenceCreateInfo::default().flags(ash::vk::FenceCreateFlags::SIGNALED);
        let fence = unsafe { device.create_fence(&fence_create_info, None) }.unwrap();

        Self { device, fence }
    }

    pub fn get_fence(&self) -> ash::vk::Fence {
        self.fence
    }
}

impl IFence for FenceAsh {
    type DeviceType = DeviceAsh;

    fn new(device: &Self::DeviceType, _info: &sjgfx_interface::FenceInfo) -> Self {
        Self::new(device)
    }
}

impl Drop for FenceAsh {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_fence(self.fence, None);
        }
    }
}
