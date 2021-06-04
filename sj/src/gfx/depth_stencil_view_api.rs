use super::Device;

pub struct DepthStencilViewInfo<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> DepthStencilViewInfo<'a> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

pub trait IDepthStencilView {
    fn new(device: &Device, info: &DepthStencilViewInfo) -> Self;
}

pub struct TDepthStencilView<T: IDepthStencilView> {
    _impl: T,
}

impl<T: IDepthStencilView> TDepthStencilView<T> {
    pub fn new(device: &Device, info: &DepthStencilViewInfo) -> Self {
        Self {
            _impl: T::new(device, info),
        }
    }
}
