use std::marker::PhantomData;
use super::super::Device;
use super::super::shader_api::ShaderInfo;
use super::super::shader_api::IShaderImpl;

pub struct ShaderImpl<'a>
{
	shader_impl: wgpu::ShaderModule,
	_marker: PhantomData<&'a i32>,
}

impl<'a> ShaderImpl<'a>
{
	pub fn get_impl(&'a self) -> &'a wgpu::ShaderModule
	{
		&self.shader_impl
	}
}

impl<'a> IShaderImpl<'a> for ShaderImpl<'a>
{
	fn new(device: &'a Device, info: &ShaderInfo) -> Self
	{
		let a = wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed("shader.wgsl"));
		let flags = wgpu::ShaderFlags::VALIDATION;		
		let shader_module = device.to_data().get_device().create_shader_module(
			&wgpu::ShaderModuleDescriptor{
				label: None,
				source: a,
				flags,
			}
		);
		
		Self{
			shader_impl: shader_module,
			_marker: PhantomData,
		}
	}
}
