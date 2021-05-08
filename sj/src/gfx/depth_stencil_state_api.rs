use super::Device;

pub struct DepthStencilStateInfo {}

impl DepthStencilStateInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait IDepthStencilStateImpl {
    fn new(device: &Device, info: &DepthStencilStateInfo) -> Self;
}

pub struct TDepthStencilState<T>
where
    T: IDepthStencilStateImpl,
{
    _impl: T,
}

impl<T: IDepthStencilStateImpl> TDepthStencilState<T> {
    fn new(device: &Device, info: &DepthStencilStateInfo) -> Self {
        Self {
            _impl: T::new(device, info),
        }
    }
}
