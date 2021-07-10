use crate::gfx::shader_api::IShaderImpl;

pub struct ShaderVk {
    _vertex_shader_module: Option<std::sync::Arc<vulkano::pipeline::shader::ShaderModule>>,
    _pixel_shader_module: Option<std::sync::Arc<vulkano::pipeline::shader::ShaderModule>>,
    _compute_shader_module: Option<std::sync::Arc<vulkano::pipeline::shader::ShaderModule>>,
}

impl<'a> IShaderImpl<'a> for ShaderVk {
    fn new(device: &'a crate::gfx::Device, info: &crate::gfx::ShaderInfo) -> Self {
        let vertex_shader_source = info.get_vertex_shader_binary().unwrap();

        let device_vk = device.to_data().get_device_impl();

        unsafe {
            let shader_module = vulkano::pipeline::shader::ShaderModule::new(
                device_vk.clone(),
                vertex_shader_source,
            )
            .unwrap();

            Self {
                _vertex_shader_module: Some(shader_module),
                _pixel_shader_module: None,
                _compute_shader_module: None,
            }
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
