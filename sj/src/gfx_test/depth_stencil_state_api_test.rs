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
