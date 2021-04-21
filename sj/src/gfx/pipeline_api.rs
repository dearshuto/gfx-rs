use super::Device;
use super::Shader;
use std::marker::PhantomData;

pub struct PipelineInfo<'a>
{
	shader: &'a Shader<'a>,
}

impl<'a> PipelineInfo<'a>
{
	pub fn new(shader: &'a Shader) -> Self{
		PipelineInfo{shader}
	}

	pub fn get_shader(&self) -> &'a Shader
	{
		self.shader
	}
}

pub trait IPipelineImpl<'a> {
	fn new(device: &'a Device, info: &PipelineInfo) -> Self;
}

pub struct TPipelineInterface<'a, T: 'a>
	where T: IPipelineImpl<'a>
{
	pipeline_impl: T,
	_marker: PhantomData<&'a T>
}

impl<'a, T: IPipelineImpl<'a>> TPipelineInterface<'a, T>	
{
	pub fn new(device: &'a Device, info: &PipelineInfo) -> Self
	{
		Self{
			pipeline_impl: T::new(device, info),
			_marker: PhantomData,
		}
	}
}
