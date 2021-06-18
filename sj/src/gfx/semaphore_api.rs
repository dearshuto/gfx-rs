use crate::gfx::Device;

pub struct SemaphoreInfo {}

impl<'a> SemaphoreInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ISemaphore<'a> {
    fn new(device: &'a Device, info: &SemaphoreInfo) -> Self;
}

pub struct TSemaphore<'a, T: ISemaphore<'a>> {
    _impl: T,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, T: ISemaphore<'a>> TSemaphore<'a, T> {
    pub fn new(device: &'a Device, info: &SemaphoreInfo) -> Self {
        Self {
            _impl: T::new(device, info),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn to_data(&self) -> &T {
        &self._impl
    }

    pub fn to_data_mut(&mut self) -> &mut T {
        &mut self._impl
    }
}
