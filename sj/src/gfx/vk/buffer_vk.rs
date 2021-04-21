use std::marker::PhantomData;
use super::super::Device;
use super::super::buffer_api::{BufferInfo, IBufferImpl};

pub struct BufferImpl<'a>
{
	_marker: PhantomData<&'a i32>,
}

impl<'a> IBufferImpl<'a> for BufferImpl<'a>
{
	fn new(device: &'a Device, info: &BufferInfo) -> Self
	{
		Self{
			_marker: PhantomData,
		}
	}
}

// impl BufferImpl<'a>
// {
//     pub fn new(device: &super::device_vk::Device, info: &BufferInfo) -> Self
//     {
// 	vulkano::buffer::CpuAccessibleBuffer::uninitialized_array(device.clone(), 124, vulkano::buffer::BufferUsage::all(), false);
// 		Self{}	
//     }
// }
