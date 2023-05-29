use std::os::raw::c_char;

pub struct PhysicalDevice<'a> {
    pub handle: ash::vk::PhysicalDevice,
    pub instance: ash::Instance,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> PhysicalDevice<'a> {
    pub(crate) fn new(handle: ash::vk::PhysicalDevice, instance: ash::Instance) -> Self {
        let properties = unsafe { instance.enumerate_device_extension_properties(handle) }.unwrap();
        for property in &properties {
            let ptr = property.extension_name.as_ptr() as *mut c_char;
            let str = unsafe { std::ffi::CStr::from_ptr(ptr) };
            println!("{:?}", str);
        }

        Self {
            handle,
            instance,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn enumerate_extension_properties(&self) -> Vec<ash::vk::ExtensionProperties> {
        unsafe {
            self.instance
                .enumerate_device_extension_properties(self.handle)
        }
        .unwrap()
    }

    pub fn get_queue_family_properties(&self) -> Vec<ash::vk::QueueFamilyProperties> {
        unsafe {
            self.instance
                .get_physical_device_queue_family_properties(self.handle)
        }
    }
}
