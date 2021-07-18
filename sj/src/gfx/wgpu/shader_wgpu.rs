use super::super::shader_api::IShaderImpl;
use super::super::shader_api::ShaderInfo;
use super::super::Device;
use std::marker::PhantomData;

pub struct ShaderImpl<'a> {
    shader_impl: wgpu::ShaderModule,
    _bind_group_layout: wgpu::BindGroupLayout,
    _marker: PhantomData<&'a i32>,
}

impl<'a> IShaderImpl<'a> for ShaderImpl<'a> {
    fn new(device: &'a Device, _info: &ShaderInfo) -> Self {
        let array = vec![0 as u32];
        let shader_source = std::borrow::Cow::Borrowed(array.as_slice());
        let shader_module = device
            .to_data()
            .get_device()
            .create_shader_module(wgpu::ShaderModuleSource::SpirV(shader_source));

        let bind_group_layout = device.to_data().get_device().create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                }],
            },
        );

        Self {
            shader_impl: shader_module,
            _bind_group_layout: bind_group_layout,
            _marker: PhantomData,
        }
    }
}

impl<'a> ShaderImpl<'a> {
    pub fn get_impl(&'a self) -> &'a wgpu::ShaderModule {
        &self.shader_impl
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self._bind_group_layout
    }
}
