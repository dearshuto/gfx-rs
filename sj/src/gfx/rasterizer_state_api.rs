use super::Device;

pub struct RasterizerStateInfo {}

impl RasterizerStateInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait IRasterizerStateImpl {
    fn new(device: &Device, info: RasterizerStateInfo) -> Self;
}

pub struct TRasterizerStateInterface<T>
where
    T: IRasterizerStateImpl,
{
    _impl: T,
}

impl<T: IRasterizerStateImpl> TRasterizerStateInterface<T> {
    pub fn new(device: &Device, info: RasterizerStateInfo) -> Self {
        Self {
            _impl: T::new(device, info),
        }
    }
}
