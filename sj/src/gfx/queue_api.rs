use super::Device;
use std::marker::PhantomData;

pub struct QueueInfo {}

impl QueueInfo {
    pub fn new() -> QueueInfo {
        QueueInfo {}
    }
}

pub trait IQueueImpl<'a> {
    fn new(device: &'a mut Device, info: &QueueInfo) -> Self;
}

pub struct TQueueInterface<'a, T: 'a>
where
    T: IQueueImpl<'a>,
{
    queue_impl: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: IQueueImpl<'a>> TQueueInterface<'a, T> {
    pub fn new(device: &'a mut Device, info: &QueueInfo) -> Self {
        TQueueInterface {
            queue_impl: T::new(device, info),
            _marker: PhantomData,
        }
    }

    pub fn to_data(&mut self) -> &mut T {
        &mut self.queue_impl
    }
}
