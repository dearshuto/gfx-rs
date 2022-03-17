use sjgfx_interface::{IQueue, QueueInfo};

pub struct TQueueBuilder<T: IQueue> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: IQueue> TQueueBuilder<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &T::DeviceType) -> T {
        T::new(device, &QueueInfo::new())
    }
}
