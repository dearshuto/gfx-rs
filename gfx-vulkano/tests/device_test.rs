
use sjgfx_interface::{IDevice, DeviceInfo};
use sjgfx_vulkano::DeviceVk;

#[test]
fn new() {
    let device_info = DeviceInfo::new();
    let _device = DeviceVk::new(&device_info);
}
