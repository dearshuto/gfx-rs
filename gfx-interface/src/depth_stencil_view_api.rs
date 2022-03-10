use crate::IDevice;

#[derive(Clone, Debug)]
pub struct DepthStencilStateInfo {
    is_depth_test_enabled: bool,
    is_depth_write_enabled: bool,
}

impl DepthStencilStateInfo {
    pub fn new() -> Self {
        Self {
            is_depth_test_enabled: false,
            is_depth_write_enabled: false,
        }
    }

    pub fn is_depth_test_enabled(&self) -> bool {
        self.is_depth_test_enabled
    }

    pub fn set_depth_test_enabled(mut self, is_enabled: bool) -> Self {
        self.is_depth_test_enabled = is_enabled;
        self
    }

    pub fn is_depth_write_enabled(&self) -> bool {
        self.is_depth_write_enabled
    }

    pub fn set_depth_write_enabled(mut self, is_enabled: bool) -> Self {
        self.is_depth_write_enabled = is_enabled;
        self
    }
}

pub trait IDepthStencilView<'a> {
    type DeviceType: IDevice;

    fn new(device: &'a Self::DeviceType, info: &DepthStencilStateInfo) -> Self;
}
