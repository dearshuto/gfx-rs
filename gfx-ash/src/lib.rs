mod buffer_ash;
mod command_buffer_ash;
mod device_ash;
mod fence_ash;
mod memory_pool_ash;
mod queue_ash;
mod shader_ash;

pub use buffer_ash::BufferAsh;
pub use command_buffer_ash::CommandBufferAsh;
pub use device_ash::DeviceAsh;
pub use fence_ash::FenceAsh;
pub use memory_pool_ash::MemoryPoolAsh;
pub use queue_ash::QueueAsh;
pub use shader_ash::ShaderAsh;

struct SharedData {
    #[allow(dead_code)]
    pub entry: ash::Entry,
    pub instance: ash::Instance,
}

static SHARED_INSTANCE: std::sync::OnceLock<SharedData> = std::sync::OnceLock::new();

pub fn initialize() {
    SHARED_INSTANCE.get_or_init(|| {
        let entry = ash::Entry::linked();
        let app_info =
            ash::vk::ApplicationInfo::builder().api_version(ash::vk::make_api_version(0, 1, 0, 0));

        let layer_names = [std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
        let layers_names_raw: Vec<*const i8> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let extension_names_raw = vec![
            ash::extensions::ext::DebugUtils::name().as_ptr(),
            // ash::vk::KhrPortabilityEnumerationFn::name().as_ptr(),
            ash::vk::KhrGetPhysicalDeviceProperties2Fn::name().as_ptr(),
        ];

        let create_info = ash::vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&layers_names_raw)
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
        initialize();
        finalize();
    }
}
