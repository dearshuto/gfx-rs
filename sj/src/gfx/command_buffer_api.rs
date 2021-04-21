use super::Device;
use std::marker::PhantomData;

pub struct CommandBufferInfo {}

impl CommandBufferInfo {
    pub fn new() -> Self {
        CommandBufferInfo {}
    }
}

pub trait ICommandBufferImpl<'a> {
    fn new(device: &'a Device, info: &CommandBufferInfo) -> Self;
}

pub struct TCommandBufferInterface<'a, T: 'a>
where
    T: ICommandBufferImpl<'a>,
{
    command_buffer_impl: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: ICommandBufferImpl<'a>> TCommandBufferInterface<'a, T> {
    pub fn new(device: &'a Device, info: &CommandBufferInfo) -> Self {
        Self {
            command_buffer_impl: T::new(device, info),
            _marker: PhantomData,
        }
    }

	pub fn to_data(&'a self) -> &'a T
	{	
		&self.command_buffer_impl
	}
}
