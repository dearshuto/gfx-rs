mod instance;
mod physical_device;

pub use instance::Instance;
pub use physical_device::PhysicalDevice;

struct DebugData {
    #[allow(dead_code)]
    debug_utils: ash::extensions::ext::DebugUtils,
    #[allow(dead_code)]
    debug_utils_messanger: ash::vk::DebugUtilsMessengerEXT,
    // #[allow(dead_code)]
    // debug_mode: Box<DebugMode>, // Validation のコールバック呼び出しで使う
}

fn create_debug_data(
    entry: &ash::Entry,
    instance: &ash::Instance,
    callback: ash::vk::PFN_vkDebugUtilsMessengerCallbackEXT,
) -> DebugData {
    let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &instance);
    let debug_utils_messanger_create_info = ash::vk::DebugUtilsMessengerCreateInfoEXT::default()
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
        .pfn_user_callback(callback)
        // .pfn_user_callback(None)
    // .user_data(debug_mode as *mut c_void);
    ;

    let debug_utils_messanger = unsafe {
        debug_utils
            .create_debug_utils_messenger(&debug_utils_messanger_create_info, None)
            .unwrap()
    };

    DebugData {
        debug_utils,
        debug_utils_messanger,
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
