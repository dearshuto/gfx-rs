use super::Device;

#[derive(Clone)]
pub struct DepthStencilStateInfo {
    _is_depth_test_enabled: bool,
    _is_depth_write_enabled: bool,
}

impl DepthStencilStateInfo {
    pub fn new() -> Self {
        Self {
            _is_depth_test_enabled: false,
            _is_depth_write_enabled: false,
        }
    }

    pub fn is_depth_test_enabled(&self) -> bool {
        self._is_depth_test_enabled
    }

    pub fn set_depth_test_enabled(mut self, is_enabled: bool) -> Self {
        self._is_depth_test_enabled = is_enabled;
        self
    }

    pub fn is_depth_write_enabled(&self) -> bool {
        self._is_depth_write_enabled
    }

    pub fn set_depth_write_enabled(mut self, is_enabled: bool) -> Self {
        self._is_depth_write_enabled = is_enabled;
        self
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
    pub fn new(device: &Device, info: &DepthStencilStateInfo) -> Self {
        Self {
            _impl: T::new(device, info),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::gfx::{DepthStencilState, DepthStencilStateInfo, Device, DeviceInfo};

    #[test]
    fn initialize() {
        let device = Device::new(&DeviceInfo::new());
        let depth_state_info = DepthStencilStateInfo::new();
        let _depth_stencil_state = DepthStencilState::new(&device, &depth_state_info);
    }
}
