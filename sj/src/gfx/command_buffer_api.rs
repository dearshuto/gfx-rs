use std::marker::PhantomData;
use super::{Buffer, Device, Pipeline};

pub struct CommandBufferInfo {}

impl CommandBufferInfo {
    pub fn new() -> Self {
        CommandBufferInfo {}
    }
}

pub trait ICommandBufferImpl<'a> {
    fn new(device: &'a Device, info: &CommandBufferInfo) -> Self;

	fn begin(&mut self);

	fn end(&mut self);
	
	fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>);

	fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32);
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

	pub fn begin(&mut self) {
		self.command_buffer_impl.begin();
	}

	pub fn end(&mut self) {
		self.command_buffer_impl.end();
	}
	
	pub fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>)
	{
		self.command_buffer_impl.set_pipeline(pipeline);
	}

	pub fn set_buffer(&mut self, _buffer: &'a Buffer)
	{
		
	}

	pub fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32)
	{
		self.command_buffer_impl.dispatch(group_count_x, group_count_y, group_count_z);
	}
	
	pub fn to_data(&'a self) -> &'a T
	{	
		&self.command_buffer_impl
	}

	pub fn to_data_mut(&'a mut self) -> &'a mut T
	{	
		&mut self.command_buffer_impl
	}	
}
