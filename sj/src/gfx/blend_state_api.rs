use super::Device;

pub struct BlendStateInfo {}

impl BlendStateInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait IBlendStateImpl {
    fn new(device: &Device, info: &BlendStateInfo) -> Self;
}

pub struct TBlendState<T>
where
    T: IBlendStateImpl,
{
    _impl: T,
}

impl<T: IBlendStateImpl> TBlendState<T> {
    pub fn new(device: &Device, info: &BlendStateInfo) -> Self {
        Self {
            _impl: T::new(device, info),
        }
    }
}
