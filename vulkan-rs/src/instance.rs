use std::ffi::{c_char, CStr};

use super::{DebugData, PhysicalDevice};

pub struct Instance {
    pub handle: ash::Instance,
    #[allow(dead_code)]
    debug_data: Option<DebugData>,
}

impl Instance {
    pub fn new() -> Self {
        Self::new_impl(|_, _| None)
    }

    pub fn new_with_debug() -> Self {
        Self::new_impl(|entry, instance| {
            Some(super::create_debug_data(
                entry,
                instance,
                Some(Self::call_debug_info),
            ))
        })
    }

    fn new_impl<F>(creator: F) -> Instance
    where
        F: FnOnce(&ash::Entry, &ash::Instance) -> Option<DebugData>,
    {
        let entry = ash::Entry::linked();
        let app_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"VulkanTriangle\0") };

        let layer_names = unsafe {
            [CStr::from_bytes_with_nul_unchecked(
                b"VK_LAYER_KHRONOS_validation\0",
            )]
        };
        let layers_names_raw: Vec<*const c_char> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let mut extension_names = Vec::default();
        extension_names.push(ash::extensions::ext::DebugUtils::NAME.as_ptr());

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            // extension_names.push(ash::vk::KhrPortabilityEnumerationFn::NAME.as_ptr());
            // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
            extension_names.push(ash::vk::KhrGetPhysicalDeviceProperties2Fn::NAME.as_ptr());
        }

        let appinfo = ash::vk::ApplicationInfo::default()
            .application_name(app_name)
            .application_version(0)
            .engine_name(app_name)
            .engine_version(0)
            .api_version(ash::vk::make_api_version(0, 1, 0, 0));

        let create_flags =
        // if cfg!(any(target_os = "macos", target_os = "ios")) {
        //     // KHR 使うときには必要かも
        //     // ash::vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        //     ash::vk::InstanceCreateFlags::default()
        // } else
        {
            ash::vk::InstanceCreateFlags::default()
        };

        let create_info = ash::vk::InstanceCreateInfo::default()
            .application_info(&appinfo)
            .enabled_layer_names(&layers_names_raw)
            .enabled_extension_names(&extension_names)
            .flags(create_flags);

        let instance: ash::Instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Instance creation error")
        };

        let debug_data = creator(&entry, &instance);
        Self {
            handle: instance,
            debug_data,
        }
    }

    pub fn enumerate_physical_devices(&self) -> Vec<PhysicalDevice> {
        let physical_devices = unsafe {
            self.handle
                .enumerate_physical_devices()
                .expect("Physical device error")
        };
        physical_devices
            .iter()
            .map(|handle| PhysicalDevice::new(handle.clone(), self.handle.clone()))
            .collect::<Vec<PhysicalDevice>>()
    }

    pub fn find_physical_device(
        &self,
        condition: &PhysicalDeviceCondition,
    ) -> PhysicalDeviceSearchResult {
        let physical_devices = self.enumerate_physical_devices();
        let (physical_device, queue_family_index) = physical_devices
            .iter()
            .find_map(|pdevice| {
                let properties = pdevice.get_queue_family_properties();
                properties.iter().enumerate().find_map(|(index, info)| {
                    let supports_graphic_and_surface =
                        info.queue_flags.contains(condition.queue_flag);
                    let is = true;
                    if supports_graphic_and_surface && is {
                        Some((pdevice.handle, index))
                    } else {
                        None
                    }
                })
            })
            .expect("Couldn't find suitable device.");

        PhysicalDeviceSearchResult {
            handle: physical_device,
            queue_family_index,
        }
    }

    unsafe extern "system" fn call_debug_info(
        message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
        message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
        _user_data: *mut std::ffi::c_void,
    ) -> u32 {
        let callback_data = *p_callback_data;
        let message_id_number: i32 = callback_data.message_id_number;

        let message_id_name = if callback_data.p_message_id_name.is_null() {
            std::borrow::Cow::from("")
        } else {
            std::ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
        };

        let message = if callback_data.p_message.is_null() {
            std::borrow::Cow::from("")
        } else {
            std::ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy()
        };

        println!(
            "{:?}:\n{:?} [{} ({})] : {}\n",
            message_severity,
            message_type,
            message_id_name,
            &message_id_number.to_string(),
            message,
        );

        ash::vk::TRUE
        // let debug_mode = user_data as *mut DebugMode;
        // match *debug_mode {
        //     DebugMode::FullAssertion => return ash::vk::TRUE,
        //     _ => return ash::vk::FALSE,
        // };
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if let Some(debug_data) = &self.debug_data {
            unsafe {
                debug_data
                    .debug_utils
                    .destroy_debug_utils_messenger(debug_data.debug_utils_messanger, None)
            };
        }

        unsafe { self.handle.destroy_instance(None) };
    }
}

pub struct PhysicalDeviceSearchResult {
    pub handle: ash::vk::PhysicalDevice,
    pub queue_family_index: usize,
}

pub struct PhysicalDeviceCondition {
    pub queue_flag: ash::vk::QueueFlags,
}

impl Default for PhysicalDeviceCondition {
    fn default() -> Self {
        Self {
            queue_flag: ash::vk::QueueFlags::COMPUTE,
        }
    }
}

#[cfg(test)]
mod tests {
    use ash::vk::Handle;

    use super::Instance;

    #[test]
    fn new() {
        let _ = Instance::new();
    }

    #[test]
    fn new_with_debug() {
        let _ = Instance::new_with_debug();
    }

    #[test]
    fn find_physical_device_for_compute() {
        let instance = Instance::new_with_debug();
        let result = instance.find_physical_device(&super::PhysicalDeviceCondition::default());
        assert!(!result.handle.is_null());
    }
}
