use sjgfx_interface::{BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess};
use sjgfx_vulkano::{BufferVk, CommandBufferVk, DeviceVk};

#[test]
fn new() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let _command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());
}

#[test]
fn set() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let buffer = BufferVk::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER)
            .set_size(std::mem::size_of::<i32>()),
    );
    let mut command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());

    command_buffer.begin();
    command_buffer.set_constant_buffer(0, &buffer);
    command_buffer.end();
}
