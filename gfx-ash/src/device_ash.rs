use std::ffi::{c_void, CStr};

use ash::{extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT, Entry};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use sjgfx_interface::{DebugMode, DeviceInfo, IDevice};

pub struct DeviceAsh {
    #[allow(dead_code)]
    entry: ash::Entry,
    instance: ash::Instance,
    debug_utils: ash::extensions::ext::DebugUtils,
    debug_utils_messanger: ash::vk::DebugUtilsMessengerEXT,
    device: ash::Device,
    physical_device: ash::vk::PhysicalDevice,
    queue: ash::vk::Queue,
    queue_family_index: usize,
    #[allow(dead_code)]
    queue_family_properties: Vec<ash::vk::QueueFamilyProperties>,
    surface: Option<ash::vk::SurfaceKHR>,
    surface_loader: Option<ash::extensions::khr::Surface>,

    #[allow(dead_code)]
    debug_mode: Box<DebugMode>, // Validation のコールバック呼び出しで使う
}

impl DeviceAsh {
    pub fn new(info: &DeviceInfo) -> Self {
        let entry = Entry::linked();

        // let extension = std::ffi::CString::new("VK_KHR_get_physical_device_properties2").unwrap();
        // let instance_extensions: &[&CStr] = &[&extension];
        let instance = Self::create_instance(&entry, &[]);

        // 物理デバイス
        let (physical_device, queue_family_index) = Self::find_physical_device(&instance);
        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        // デバイスとキュー
        // let device_extension = std::ffi::CString::new("VK_KHR_portability_subset").unwrap();
        // let device_extensions: &[&CStr] = &[&device_extension];
        let (device, queue) = Self::create_device_and_queue(
            &instance,
            physical_device,
            queue_family_index as u32,
            &[],
        );

        // デバッグ機能
        let debug_mode = Box::new(info.get_debug_mode());
        let (debug_utils, debug_utils_messanger) =
            Self::create_debug_instance(&entry, &instance, &*debug_mode);

        Self {
            entry,
            instance,
            debug_utils,
            debug_utils_messanger,
            device,
            physical_device,
            queue,
            queue_family_index,
            queue_family_properties,
            surface: None,
            surface_loader: None,
            debug_mode,
        }
    }

    pub fn new_with_surface<
        T: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle,
    >(
        info: &DeviceInfo,
        window: &T,
    ) -> Self {
        let entry = Entry::linked();

        // let extension = std::ffi::CString::new("VK_KHR_get_physical_device_properties2").unwrap();
        let additional_ectensions =
            ash_window::enumerate_required_extensions(window.raw_display_handle()).unwrap();
        // additional_ectensions.push(&extension);
        let instance = Self::create_instance(&entry, &additional_ectensions);

        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )
        }
        .unwrap();
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);
        let (physical_device, queue_family_index) =
            Self::find_physical_device_with_predicate(&instance, |physical_device, index| {
                let is_surface_supported = unsafe {
                    surface_loader.get_physical_device_surface_support(
                        *physical_device,
                        index as u32,
                        surface,
                    )
                }
                .unwrap();
                is_surface_supported
            });

        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        // デバイスとキュー
        // let device_extension = std::ffi::CString::new("VK_KHR_portability_subset").unwrap();
        let extensions = [ash::extensions::khr::Swapchain::name()];
        let (device, queue) = Self::create_device_and_queue(
            &instance,
            physical_device,
            queue_family_index as u32,
            &extensions,
        );

        // デバッグ機能
        let debug_mode = Box::new(info.get_debug_mode());
        let (debug_utils, debug_utils_messanger) =
            Self::create_debug_instance(&entry, &instance, &*debug_mode);

        Self {
            entry,
            instance,
            debug_utils,
            debug_utils_messanger,
            device,
            physical_device,
            queue,
            queue_family_index,
            queue_family_properties,
            surface: Some(surface),
            surface_loader: Some(surface_loader),
            debug_mode,
        }
    }

    pub fn get_entry(&self) -> &ash::Entry {
        &self.entry
    }

    pub fn get_instance(&self) -> &ash::Instance {
        &self.instance
    }

    pub fn get_device(&self) -> ash::Device {
        self.device.clone()
    }

    pub fn get_device_ref(&self) -> &ash::Device {
        &self.device
    }

    pub fn get_physical_device(&self) -> ash::vk::PhysicalDevice {
        self.physical_device
    }

    pub fn get_queue_handle(&self) -> ash::vk::Queue {
        self.queue
    }

    pub fn get_queue_family_index(&self) -> u32 {
        self.queue_family_index as u32
    }

    pub fn get_surface_loader(&self) -> &ash::extensions::khr::Surface {
        self.surface_loader.as_ref().unwrap()
    }

    pub fn get_surface(&self) -> ash::vk::SurfaceKHR {
        self.surface.unwrap()
    }

    fn create_instance(entry: &ash::Entry, additional_extensions: &[*const i8]) -> ash::Instance {
        let app_name = std::ffi::CString::new("VulkanTriangle").unwrap();
        let appinfo = ash::vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(0)
            .engine_name(&app_name)
            .engine_version(0)
            .api_version(ash::vk::make_api_version(0, 1, 0, 0));

        let layer_names = [std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
        let layers_names_raw: Vec<*const i8> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let mut extension_names_raw = vec![
            ash::extensions::ext::DebugUtils::name().as_ptr(),
            // unsafe {
            //     CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_physical_device_properties2\0")
            // }.as_ptr(),
        ];
        for additional_extension in additional_extensions {
            extension_names_raw.push(*additional_extension);
        }

        let create_info = ash::vk::InstanceCreateInfo::builder()
            .application_info(&appinfo)
            .enabled_layer_names(&layers_names_raw)
            .enabled_extension_names(&extension_names_raw);

        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Instance creation error")
        };

        instance
    }

    fn find_physical_device(instance: &ash::Instance) -> (ash::vk::PhysicalDevice, usize) {
        Self::find_physical_device_with_predicate(instance, |_x, _y| true)
    }

    fn find_physical_device_with_predicate<F>(
        instance: &ash::Instance,
        func: F,
    ) -> (ash::vk::PhysicalDevice, usize)
    where
        F: Fn(&ash::vk::PhysicalDevice, usize) -> bool,
    {
        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Physical device error")
        };

        let (physical_device, queue_family_index) = physical_devices
            .iter()
            .map(|pdevice| {
                let properties =
                    unsafe { instance.get_physical_device_queue_family_properties(*pdevice) };
                properties
                    .iter()
                    .enumerate()
                    .filter_map(|(index, info)| {
                        let supports_graphic_and_surface =
                            info.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS);
                        let is = func(pdevice, index);
                        if supports_graphic_and_surface && is {
                            Some((*pdevice, index))
                        } else {
                            None
                        }
                    })
                    .next()
            })
            .flatten()
            .next()
            .expect("Couldn't find suitable device.");

        (physical_device, queue_family_index)
    }

    fn create_device_and_queue(
        instance: &ash::Instance,
        physical_device: ash::vk::PhysicalDevice,
        queue_family_index: u32,
        extension_name: &[&CStr],
    ) -> (ash::Device, ash::vk::Queue) {
        let device_extension_names_raw = extension_name
            .iter()
            .map(|x| x.as_ptr())
            .collect::<Vec<_>>();

        let features = ash::vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };
        let priorities = [1.0];
        let queue_info = [ash::vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .queue_priorities(&priorities)
            .build()];
        let device_create_info = ash::vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_info)
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);
        let device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .unwrap()
        };

        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        (device, queue)
    }

    fn create_debug_instance(
        entry: &ash::Entry,
        instance: &ash::Instance,
        debug_mode: *const DebugMode,
    ) -> (DebugUtils, DebugUtilsMessengerEXT) {
        let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &instance);
        let debug_utils_messanger_create_info =
            ash::vk::DebugUtilsMessengerCreateInfoEXT::builder()
                .message_severity(
                    ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                        | ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                        | ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                )
                .message_type(
                    ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                        | ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .pfn_user_callback(Some(Self::aa))
                .user_data(debug_mode as *mut c_void)
                .build();

        let debug_utils_messanger = unsafe {
            debug_utils
                .create_debug_utils_messenger(&debug_utils_messanger_create_info, None)
                .unwrap()
        };

        (debug_utils, debug_utils_messanger)
    }

    unsafe extern "system" fn aa(
        message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
        message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
        user_data: *mut std::ffi::c_void,
    ) -> u32 {
        let callback_data = *p_callback_data;
        let message_id_number: i32 = callback_data.message_id_number as i32;

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

        let debug_mode = user_data as *mut DebugMode;
        match *debug_mode {
            DebugMode::FullAssertion => return ash::vk::TRUE,
            _ => return ash::vk::FALSE,
        };
    }
}

impl Drop for DeviceAsh {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            if let Some(surface) = self.surface {
                self.surface_loader
                    .as_ref()
                    .unwrap()
                    .destroy_surface(surface, None);
            }
            self.device.destroy_device(None);
            self.debug_utils
                .destroy_debug_utils_messenger(self.debug_utils_messanger, None);
            self.instance.destroy_instance(None);
        }
    }
}

impl IDevice for DeviceAsh {
    fn new(info: &DeviceInfo) -> Self {
        Self::new(info)
    }

    fn new_with_handle<T>(info: &DeviceInfo, raw_handle: &T) -> Self
    where
        T: HasRawWindowHandle + HasRawDisplayHandle,
    {
        Self::new_with_surface(info, raw_handle)
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{DebugMode, DeviceInfo};

    use crate::DeviceAsh;

    #[test]
    fn new() {
        let _device = DeviceAsh::new(&DeviceInfo::new());
    }

    #[test]
    fn new_full_assertion() {
        let _device = DeviceAsh::new(&DeviceInfo::new().set_debug_mode(DebugMode::FullAssertion));
    }
}
