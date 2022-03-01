use sjgfx_interface::{IDevice, DeviceInfo, CommandBufferInfo, BufferInfo};
use sjgfx_vulkano::{CommandBufferVk, DeviceVk, BufferVk};

#[test]
fn new()
{
    let device = DeviceVk::new(&DeviceInfo::new());
    let _command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());
}

#[test]
fn set()
{
    let device = DeviceVk::new(&DeviceInfo::new());
    let buffer = BufferVk::new::<i32>(&device, &BufferInfo::new());
    let mut command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());

    command_buffer.begin();
    command_buffer.set_constant_buffer(0, &buffer);
    command_buffer.end();
}
