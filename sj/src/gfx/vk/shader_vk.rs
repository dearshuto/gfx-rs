use super::super::Device;
use super::super::shader_api::{ShaderInfo, IShaderImpl};
use std::marker::PhantomData;

pub struct ShaderImpl<'a>
{
	_marker: PhantomData<&'a u32>,
}

impl<'a> IShaderImpl<'a> for ShaderImpl<'a>
{
	fn new(device: &'a Device, info: &ShaderInfo) -> Self
	{
		Self{ _marker: PhantomData }
	}
}
