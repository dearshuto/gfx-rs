use super::super::Device;
use super::super::command_buffer_api::{CommandBufferInfo, ICommandBufferImpl};
use std::marker::PhantomData;

pub struct CommandBufferImpl<'a>
{
	_marker: PhantomData<&'a u32>,
}

impl<'a> ICommandBufferImpl<'a> for CommandBufferImpl<'a>
{
    fn new(device: &'a Device, info: &CommandBufferInfo) -> Self
	{
		Self{
			_marker: PhantomData
		}
	}
}
