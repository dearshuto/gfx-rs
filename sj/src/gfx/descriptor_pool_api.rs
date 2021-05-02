use super::Device;
use super::Shader;

pub struct DescriptorPoolInfo<'a> {
	_shader: &'a Shader<'a>,
}

impl<'a> DescriptorPoolInfo<'a>
{
	pub fn get_shader(&'a self) -> &'a Shader
	{
		&self._shader
	}
}

pub trait IDescriptorPoolImpl
{
	fn new(device: &Device, info: &DescriptorPoolInfo) -> Self;
}

pub struct TDescriptorInterface<T>
	where T: IDescriptorPoolImpl
{
	descriptor_pool_impl: T,
}
