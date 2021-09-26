use crate::gfx::shader_api::{IShaderImpl, ShaderInfo};
use crate::gfx::Device;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct ShaderImpl<'a> {
    _vertex_shader_module: Option<Arc<wgpu::ShaderModule>>,
    _pixel_shader_module: Option<Arc<wgpu::ShaderModule>>,
    _compute_shader_module: Option<Arc<wgpu::ShaderModule>>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ShaderImpl<'a> {
    pub fn clone_vertex_shader_module(&self) -> Arc<wgpu::ShaderModule> {
        self._vertex_shader_module.as_ref().unwrap().clone()
    }

    pub fn clone_pixel_shader_module(&self) -> Arc<wgpu::ShaderModule> {
        self._pixel_shader_module.as_ref().unwrap().clone()
    }

    pub fn clone_compute_shader_module(&self) -> Arc<wgpu::ShaderModule> {
        self._compute_shader_module.as_ref().unwrap().clone()
    }

    fn create_shader_module(
        device: &wgpu::Device,
        sprv_binary_opt: &Option<&[u8]>,
    ) -> Option<Arc<wgpu::ShaderModule>> {
        match sprv_binary_opt {
            Some(sprv_binary) => Some(Arc::new(device.create_shader_module(
                &wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::util::make_spirv(sprv_binary),
                },
            ))),
            None => None,
        }
    }
}

impl<'a> IShaderImpl<'a> for ShaderImpl<'a> {
    fn new(device: &'a Device, info: &ShaderInfo) -> Self {
        let device_impl = device.to_data().get_device();
        let vertex_shader_module =
            ShaderImpl::create_shader_module(device_impl, info.get_vertex_shader_binary());
        let pixel_shader_module =
            ShaderImpl::create_shader_module(device_impl, info.get_pixel_shader_binary());
        let compute_shader_module =
            ShaderImpl::create_shader_module(device_impl, info.get_compute_shader_binary());

        Self {
            _vertex_shader_module: vertex_shader_module,
            _pixel_shader_module: pixel_shader_module,
            _compute_shader_module: compute_shader_module,
            _marker: PhantomData,
        }
    }
}
