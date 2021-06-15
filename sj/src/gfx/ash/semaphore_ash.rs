use crate::gfx::{semaphore_api::ISemaphore, Device};
use ash::version::DeviceV1_0;

pub struct SemaphoreAsh<'a> {
    _device: &'a Device,
    _semaphore: ash::vk::Semaphore,
}

impl<'a> SemaphoreAsh<'a> {
    pub fn get_semaphore(&mut self) -> ash::vk::Semaphore {
        self._semaphore
    }
}

impl<'a> ISemaphore<'a> for SemaphoreAsh<'a> {
    fn new(device: &'a Device, _info: &crate::gfx::semaphore_api::SemaphoreInfo) -> Self {
        let device_ash = device.to_data().get_device();
        let semaphore_create_info = ash::vk::SemaphoreCreateInfo::default();

        unsafe {
            let semaphore = device_ash
                .create_semaphore(&semaphore_create_info, None)
                .unwrap();

            Self {
                _device: device,
                _semaphore: semaphore,
            }
        }
    }
}

impl<'a> Drop for SemaphoreAsh<'a> {
    fn drop(&mut self) {
        let device_ash = self._device.to_data().get_device();

        unsafe {
            device_ash.destroy_semaphore(self._semaphore, None);
        }
    }
}
