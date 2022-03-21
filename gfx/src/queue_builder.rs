use sjgfx_interface::{IQueue, QueueInfo};

use crate::api::IApi;

pub struct TQueueBuilder<T: IApi> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: IApi> TQueueBuilder<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &T::Device) -> T::Queue {
        T::Queue::new(device, &QueueInfo::new())
    }
}
