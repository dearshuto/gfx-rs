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

impl Drop for SemaphoreAsh {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_semaphore(self.semaphore, None);
        }
    }
}
