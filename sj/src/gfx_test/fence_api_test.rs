use crate::gfx::{Device, DeviceInfo, Fence, FenceInfo};

#[test]
fn initialize() {
    let device = Device::new(&DeviceInfo::new());
    let _fence = Fence::new(&device, &FenceInfo {});
}
