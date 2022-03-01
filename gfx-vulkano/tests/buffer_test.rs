use sjgfx_interface::{IDevice, DeviceInfo, BufferInfo};
use sjgfx_vulkano::{DeviceVk, BufferVk};

#[test]
fn new() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let _buffer = BufferVk::new::<i32>(&device, &BufferInfo::new());
}

#[test]
fn new_buffer() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let _buffer = BufferVk::new::<i32>(&device, &BufferInfo::new());
}

#[test]
fn new_array_buffer() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let _buffer = BufferVk::new_as_array::<i32>(&device, &BufferInfo::new());
}
