use super::super::device_api::{DeviceInfo, TDeviceImpl};
pub use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};

pub struct DeviceImpl {
    _entry: ash::Entry,
    _instance: ash::Instance,
    _debug_utils: ash::extensions::ext::DebugUtils,
    _debug_utils_messanger: ash::vk::DebugUtilsMessengerEXT,
    pub _device: ash::Device,
	_physical_device: ash::vk::PhysicalDevice,
    _queue: ash::vk::Queue,
    _queue_family_index: u32,
    _queue_family_properties: Vec<ash::vk::QueueFamilyProperties>,
}

impl DeviceImpl {
	pub fn get_instance(&self) -> &ash::Instance {
		&self._instance
	}
	
    pub fn get_device(&self) -> &ash::Device {
        &self._device
    }

	pub fn get_physical_device(&self) -> &ash::vk::PhysicalDevice {
		&self._physical_device
	}
	
    pub fn get_queue_family_index(&self) -> u32 {
        self._queue_family_index
    }
}

impl TDeviceImpl for DeviceImpl {
    fn new(_info: &DeviceInfo) -> Self {
        unsafe {
            let app_name = std::ffi::CString::new("VulkanTriangle").unwrap();
            let entry = ash::Entry::new().unwrap();
            let appinfo = ash::vk::ApplicationInfo::builder()
                .application_name(&app_name)
                .application_version(0)
                .engine_name(&app_name)
                .engine_version(0)
                .api_version(ash::vk::make_version(1, 0, 0));

            let layer_names = [std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
            let layers_names_raw: Vec<*const i8> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

            let surface_extensions = vec![
                ash::extensions::khr::Surface::name(),
                //				ash::extensions::khr::XlibSurface::name(),
                //				ash::extensions::khr::WaylandSurface::name(),
                //				ash::extensions::khr::XcbSurface::name(),
                //				ash::extensions::ext::MetalSurface::name()
            ];
            let mut extension_names_raw = surface_extensions
                .iter()
                .map(|ext| ext.as_ptr())
                .collect::<Vec<_>>();
            extension_names_raw.push(ash::extensions::ext::DebugUtils::name().as_ptr());

            let create_info = ash::vk::InstanceCreateInfo::builder()
                .application_info(&appinfo)
                .enabled_layer_names(&layers_names_raw)
                .enabled_extension_names(&extension_names_raw);

            let instance = entry
                .create_instance(&create_info, None)
                .expect("Instance creation error");

            let physical_devices = instance
                .enumerate_physical_devices()
                .expect("Physical device error");
            let _surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);

            let (physical_device, queue_family_index) = physical_devices
                .iter()
                .find_map(|physical_device| {
                    instance
                        .get_physical_device_queue_family_properties(*physical_device)
                        .iter()
                        .enumerate()
                        .find_map(|(index, ref info)| {
                            let supports_graphic_and_surface =
                                info.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS);
                            if supports_graphic_and_surface {
                                Some((*physical_device, index))
                            } else {
                                None
                            }
                        })
                })
                .unwrap();

            let queue_family_properties =
                instance.get_physical_device_queue_family_properties(physical_device);
            let queue_family_index = queue_family_index as u32;
            let device_extension_names_raw = [ash::extensions::khr::Swapchain::name().as_ptr()];
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
            let device = instance
                .create_device(physical_device, &device_create_info, None)
                .unwrap();
            let queue = device.get_device_queue(queue_family_index, 0);

            let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &instance);
            let debug_utils_messanger_create_info =
                ash::vk::DebugUtilsMessengerCreateInfoEXT::builder()
                    .message_severity(
                        ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                            | ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                            | ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                    )
                    .message_type(ash::vk::DebugUtilsMessageTypeFlagsEXT::all())
                    .pfn_user_callback(Some(aa))
                    .build();

            let debug_utils_messanger = debug_utils
                .create_debug_utils_messenger(&debug_utils_messanger_create_info, None)
                .unwrap();

            Self {
                _entry: entry,
                _instance: instance,
                _debug_utils: debug_utils,
                _debug_utils_messanger: debug_utils_messanger,
                _device: device,
                _queue: queue,
				_physical_device: physical_device,
                _queue_family_index: queue_family_index,
                _queue_family_properties: queue_family_properties,
            }
        }
    }
}

unsafe extern "system" fn aa(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::ffi::c_void,
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
    ash::vk::FALSE
}

impl Drop for DeviceImpl {
    fn drop(&mut self) {
        unsafe {
            self._device.device_wait_idle().unwrap();
            self._device.destroy_device(None);
            self._debug_utils
                .destroy_debug_utils_messenger(self._debug_utils_messanger, None);
            self._instance.destroy_instance(None);
        }
    }
}
