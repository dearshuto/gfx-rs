use ash::RawPtr;
use vulkano::VulkanObject;

use crate::DeviceVk;

pub struct ShaderObject {
    device: ash::vk::Device,
    fp: ash::vk::ExtShaderObjectFn,
    shader_handle: ash::vk::ShaderEXT,
}

impl ShaderObject {
    pub fn new(device: &DeviceVk) -> Self {
        let handle = device.clone_device().handle();
        let instance = device.clone_device().instance().handle();

        let fp = ash::vk::ExtShaderObjectFn::load(|name| unsafe {
            let addr = device
                .clone_device()
                .instance()
                .library()
                .get_instance_proc_addr(instance, name.as_ptr());

            std::mem::transmute(addr)
        });

        let shader_create_infos = vec![];
        let mut shaders = Vec::with_capacity(1);
        unsafe {
            (fp.create_shaders_ext)(
                handle,
                1, /*len*/
                shader_create_infos.as_ptr(),
                None.as_raw_ptr(),
                shaders.as_mut_ptr(),
            )
            .result()
            .unwrap()
        };

        Self {
            device: handle,
            fp,
            shader_handle: shaders[0],
        }
    }
}

impl Drop for ShaderObject {
    fn drop(&mut self) {
        unsafe { (self.fp.destroy_shader_ext)(self.device, self.shader_handle, None.as_raw_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::DeviceInfo;

    use super::ShaderObject;

    #[test]
    pub fn new() {
        let device = crate::DeviceVk::new(&DeviceInfo::new());
        let _shader_object = ShaderObject::new(&device);
    }
}
