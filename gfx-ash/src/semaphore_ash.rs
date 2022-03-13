use sjgfx_interface::ISemaphore;

use crate::DeviceAsh;

pub struct SemaphoreAsh {
    device: ash::Device,
    semaphore: ash::vk::Semaphore,
}

impl SemaphoreAsh {
    pub fn new(device: &DeviceAsh) -> Self {
        let device = device.get_device();

        let semaphore_create_info = ash::vk::SemaphoreCreateInfo::default();
        let semaphore = unsafe { device.create_semaphore(&semaphore_create_info, None) }.unwrap();

        Self { device, semaphore }
    }

    pub fn get_semaphore(&self) -> ash::vk::Semaphore {
        self.semaphore
    }
}

impl ISemaphore for SemaphoreAsh {
    type DeviceType = DeviceAsh;

    fn new(device: &Self::DeviceType, _info: &sjgfx_interface::SemaphoreInfo) -> Self {
        Self::new(device)
    }
}

impl Drop for SemaphoreAsh {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_semaphore(self.semaphore, None);
        }
    }
}
