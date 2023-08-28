use sjgfx_interface::DeviceInfo;

pub struct DeviceAsh {
    device: ash::Device,

    physical_device: ash::vk::PhysicalDevice,
}

impl DeviceAsh {
    pub fn new(_info: &DeviceInfo) -> Self {
        let instance = &crate::SHARED_INSTANCE.get().as_ref().unwrap().instance;

        let Ok(physical_devices) = (unsafe { instance.enumerate_physical_devices() })  else {
          panic!()  
        };
        let Some((physical_device, index)) = physical_devices
            .iter()
                .map(|physical_device| {
                    let properties =
                        unsafe{ instance.get_physical_device_queue_family_properties(*physical_device)};
                    properties
                        .iter()
                        .enumerate()
                        .find_map(|(index, info)| {
                            if !info.queue_flags.contains(ash::vk::QueueFlags::COMPUTE) {
                                return None;
                            }

                            return Some((physical_device.clone(), index));
                        })
                }).flatten().next() else {
            panic!()
        };
        
            let device_extension_names_raw = [
                #[cfg(any(target_os = "macos", target_os = "ios"))]
                ash::vk::KhrPortabilitySubsetFn::name().as_ptr(),

            ];
        let device_create_info = ash::vk::DeviceCreateInfo::builder()
            .queue_create_infos(&[ash::vk::DeviceQueueCreateInfo::builder().queue_family_index(index as u32).queue_priorities(&[1.0]).build()])
            .enabled_extension_names(&device_extension_names_raw)
            .build();
        let device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .unwrap()
        };


        Self { device , physical_device}
    }

    pub fn get_physical_device_handle(&self) -> ash::vk::PhysicalDevice {
        self.physical_device
    }

    pub fn handle(&self) -> ash::Device {
        self.device.clone()
    }
}

impl Drop for DeviceAsh {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
