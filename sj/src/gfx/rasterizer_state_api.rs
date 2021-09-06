use super::Device;

#[derive(Clone, Copy, Debug)]
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

#[cfg(test)]
mod tests {
    use super::super::super::gfx::{Device, DeviceInfo, RasterizerState, RasterizerStateInfo};

    #[test]
    fn initialize() {
        let device = Device::new(&DeviceInfo::new());
        let rasterizer_state_info = RasterizerStateInfo::new();
        let _rasterizer_state = RasterizerState::new(&device, rasterizer_state_info);
    }
}
