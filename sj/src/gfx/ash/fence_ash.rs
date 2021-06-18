use ash::version::DeviceV1_0;

use crate::gfx::{fence_api::IFence, Device};

pub struct FenceAsh<'a> {
    _device: &'a Device,
    _fence: ash::vk::Fence,
}

impl<'a> FenceAsh<'a> {
    pub fn get_fence(&self) -> ash::vk::Fence {
        self._fence
    }
}

impl<'a> IFence<'a> for FenceAsh<'a> {
    fn new(device: &'a crate::gfx::Device, _info: &crate::gfx::fence_api::FenceInfo) -> Self {
        let device_ash = device.to_data().get_device();
        let fence_create_info = ash::vk::FenceCreateInfo::builder()
            .flags(ash::vk::FenceCreateFlags::SIGNALED)
            .build();

        unsafe {
            let fence = device_ash.create_fence(&fence_create_info, None).unwrap();

            Self {
                _device: device,
                _fence: fence,
            }
        }
    }
}

impl<'a> Drop for FenceAsh<'a> {
    fn drop(&mut self) {
        let device_ash = self._device.to_data().get_device();

        unsafe {
            device_ash.destroy_fence(self._fence, None);
        }
    }
}
