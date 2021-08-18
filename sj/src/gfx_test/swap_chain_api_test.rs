use crate::gfx::{Device, DeviceInfo};

#[test]
fn initialize() {
    let device_info = DeviceInfo::new();
    let _device = Device::new(&device_info);

    //let swap_chain = crate::gfx::SwapChainInfo::new();
}
