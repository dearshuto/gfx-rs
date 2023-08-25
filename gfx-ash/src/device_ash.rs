use sjgfx_interface::DeviceInfo;

pub struct DeviceAsh {
    device: ash::Device,
}

impl DeviceAsh {
    pub fn new(_info: &DeviceInfo) -> Self {
        let instance = &crate::SHARED_INSTANCE.get().as_ref().unwrap().instance;
        let physical_device = unsafe {
            instance
                .enumerate_physical_devices()
                .iter()
                .next()
                .unwrap()
                .clone()
        };

        let device_create_info = ash::vk::DeviceCreateInfo::default();
        let device = unsafe {
            instance
                .create_device(physical_device[0], &device_create_info, None)
                .unwrap()
        };

        Self { device }
    }
}

impl Drop for DeviceAsh {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
