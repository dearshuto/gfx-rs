use super::Device;

pub struct FenceInfo {}

pub trait IFence<'a> {
    fn new(device: &'a Device, info: &FenceInfo) -> Self;
}

pub struct TFence<'a, T: IFence<'a>> {
    _impl: T,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, T: IFence<'a>> TFence<'a, T> {
    pub fn new(device: &'a Device, info: &FenceInfo) -> Self {
        Self {
            _impl: T::new(device, info),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn to_data(&self) -> &T {
        &self._impl
    }
}
