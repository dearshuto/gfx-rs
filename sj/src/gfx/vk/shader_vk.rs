use crate::gfx::shader_api::IShaderImpl;
use vulkano::pipeline::shader::ShaderModule;

pub struct ShaderVk {
    _vertex_shader_module: Option<std::sync::Arc<vulkano::pipeline::shader::ShaderModule>>,
    _pixel_shader_module: Option<std::sync::Arc<vulkano::pipeline::shader::ShaderModule>>,
    _compute_shader_module: Option<std::sync::Arc<vulkano::pipeline::shader::ShaderModule>>,
}

impl<'a> IShaderImpl<'a> for ShaderVk {
    fn new(device: &'a crate::gfx::Device, info: &crate::gfx::ShaderInfo) -> Self {
        let device_vk = device.to_data().get_device_impl();

        let vertex_shader_module = match info.get_vertex_shader_binary() {
            Some(vertex_shader_source) => unsafe {
                Some(ShaderModule::new(device_vk.clone(), vertex_shader_source).unwrap())
            },
            None => None,
        };
        let pixel_shader_module = match info.get_pixel_shader_binary() {
            Some(pixel_shader_source) => unsafe {
                Some(ShaderModule::new(device_vk.clone(), pixel_shader_source).unwrap())
            },
            None => None,
        };
        let compute_shader_module = match info.get_compute_shader_binary() {
            Some(compute_shader_source) => unsafe {
                Some(ShaderModule::new(device_vk.clone(), compute_shader_source).unwrap())
            },
            None => None,
        };

        Self {
            _vertex_shader_module: vertex_shader_module,
            _pixel_shader_module: pixel_shader_module,
            _compute_shader_module: compute_shader_module,
        }
    }
}

impl ShaderVk {
    pub fn get_vertex_shader_module(
        &self,
    ) -> &std::sync::Arc<vulkano::pipeline::shader::ShaderModule> {
        self._vertex_shader_module.as_ref().unwrap()
    }

    pub fn get_pixel_shader_module(
        &self,
    ) -> &std::sync::Arc<vulkano::pipeline::shader::ShaderModule> {
        self._pixel_shader_module.as_ref().unwrap()
    }

    pub fn get_compute_shader_module(
        &self,
    ) -> &std::sync::Arc<vulkano::pipeline::shader::ShaderModule> {
        self._compute_shader_module.as_ref().unwrap()
    }
}
