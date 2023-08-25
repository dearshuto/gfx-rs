mod device_ash;

pub use device_ash::DeviceAsh;

struct SharedData {
    #[allow(dead_code)]
    pub entry: ash::Entry,
    pub instance: ash::Instance,
}

static SHARED_INSTANCE: std::sync::OnceLock<SharedData> = std::sync::OnceLock::new();

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn initialize() {
    SHARED_INSTANCE.get_or_init(|| {
        let entry = ash::Entry::linked();
        let app_info =
            ash::vk::ApplicationInfo::builder().api_version(ash::vk::make_api_version(0, 1, 0, 0));

        let extension_names_raw = vec![ash::extensions::ext::DebugUtils::name().as_ptr()];
        let create_info = ash::vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            // .enabled_layer_names(&layers_names_raw)
            .enabled_extension_names(&extension_names_raw);

        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Instance creation error")
        };

        SharedData { entry, instance }
    });
}

pub fn finalize() {
    let shared_data = SHARED_INSTANCE.get().unwrap();
    unsafe {
        shared_data.instance.destroy_instance(None);
    }
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
