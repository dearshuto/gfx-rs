use super::super::shader_api::IShaderImpl;
use super::super::shader_api::ShaderInfo;
use super::super::Device;
use std::marker::PhantomData;

pub struct ShaderImpl<'a> {
    shader_impl: wgpu::ShaderModule,
    _marker: PhantomData<&'a i32>,
}

impl<'a> ShaderImpl<'a> {
    pub fn get_impl(&'a self) -> &'a wgpu::ShaderModule {
        &self.shader_impl
    }
}

impl<'a> IShaderImpl<'a> for ShaderImpl<'a> {
    fn new(device: &'a Device, info: &ShaderInfo) -> Self {
        let shader_source = std::borrow::Cow::Borrowed(info.get_shader_binary());
        let shader_module = device
            .to_data()
            .get_device()
            .create_shader_module(&wgpu::ShaderModuleSource::Sprv(shader_source));

        Self {
            shader_impl: shader_module,
            _marker: PhantomData,
        }
    }
}
