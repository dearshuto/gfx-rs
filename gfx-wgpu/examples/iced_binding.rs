use sjgfx_interface::{IDevice, DeviceInfo};
use sjwgpu_wgpu::DeviceWgpu;

use iced_wgpu;

fn main() {
    let mut  device = DeviceWgpu::new(&DeviceInfo::new());
    let _ = iced_wgpu::Backend::new(device.get_device_mut(), iced_wgpu::settings::Settings::default(), wgpu::TextureFormat::Rgba8Unorm);
}
