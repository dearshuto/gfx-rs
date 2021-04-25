use super::Device;
use std::marker::PhantomData;


pub struct ShaderInfo<'a> {
	_shader_binary: Option<&'a [u8]>,
}

impl<'a> ShaderInfo<'a>{
	pub fn new() -> Self{
		Self{
			_shader_binary: None,
		}
	}

	pub fn get_shader_binary(&self) -> &'a[u8]
	{
		self._shader_binary.unwrap()
	}
	
	pub fn set_shader_binary(mut self, shader_binary: &'a [u8]) -> Self
	{
		self._shader_binary = Some(shader_binary);
		self
	}
}

pub trait IShaderImpl<'a>
{
	fn new(device: &'a Device, info: &ShaderInfo) -> Self;
}

pub struct TShaderInterface<'a, T: 'a>
	where T: IShaderImpl<'a>
{
	shader_impl: T,
	_marker: PhantomData<&'a T>,
}

impl<'a, T: IShaderImpl<'a>> TShaderInterface<'a, T>
{
	pub fn new(device: &'a Device, info: &ShaderInfo) -> Self
	{
		Self{
			shader_impl: T::new(device, info),
			_marker: PhantomData,
		}
	}

	pub fn to_data(&'a self) -> &'a T
	{
		&self.shader_impl
	}
}
