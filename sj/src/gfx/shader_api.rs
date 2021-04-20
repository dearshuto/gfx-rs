use super::Device;
use std::marker::PhantomData;


pub struct ShaderInfo{}

impl ShaderInfo{
	pub fn new() -> Self{
		Self{}
	}
}

pub trait IShaderImpl<'a>
{
	fn new(device: &'a mut Device, info: &ShaderInfo) -> Self;
}

pub struct TShaderInterface<'a, T: 'a>
	where T: IShaderImpl<'a>
{
	shader_impl: T,
	_marker: PhantomData<&'a T>,
}

impl<'a, T: IShaderImpl<'a>> TShaderInterface<'a, T>
{
	pub fn new(device: &'a mut Device, info: &ShaderInfo) -> Self
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
