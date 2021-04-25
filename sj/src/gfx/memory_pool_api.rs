use std::marker::PhantomData;
use super::Device;

pub struct MemoryPoolInfo
{
	
}

impl MemoryPoolInfo{
	pub fn new() -> Self{
		Self{}
	}

	pub fn get_size(&self) -> u64{
		128
	}
}

pub trait IMemoryPoolImpl<'a>
{
	fn new(device: &'a Device, info: &MemoryPoolInfo) -> Self;
}

pub struct TMemoryPoolInterface<'a, T>
	where T: IMemoryPoolImpl<'a>
{
	_memory_pool_impl: T,
	_marker: PhantomData<&'a u32>,
}

impl<'a, T: IMemoryPoolImpl<'a>> TMemoryPoolInterface<'a, T>
{
	pub fn new(device: &'a Device, info: &MemoryPoolInfo) -> Self
	{
		Self{
			_memory_pool_impl: T::new(device, info),
			_marker: PhantomData,
		}
	}

	pub fn to_data(&self) -> &T
	{
		&self._memory_pool_impl
	}
}
