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
