use super::Device;
use std::marker::PhantomData;

pub struct BufferInfo {}

impl BufferInfo {
    pub fn new() -> Self {
        BufferInfo {}
    }
}

pub trait IBufferImpl<'a> {
    fn new(device: &'a mut Device, info: &BufferInfo) -> Self;
}

pub struct TBufferInterface<'a, T: 'a>
where
    T: IBufferImpl<'a>,
{
    buffer_impl: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: IBufferImpl<'a>> TBufferInterface<'a, T> {
    pub fn new(device: &'a mut Device, info: &BufferInfo) -> Self {
        Self {
            buffer_impl: T::new(device, info),
            _marker: PhantomData,
        }
    }
}
