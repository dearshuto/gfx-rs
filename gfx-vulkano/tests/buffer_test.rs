use sjgfx_interface::{BufferInfo, DeviceInfo, GpuAccess, IDevice};
use sjgfx_vulkano::{BufferVk, DeviceVk};

struct Data {
    #[allow(dead_code)]
    pub value0: i32,
    #[allow(dead_code)]
    pub value1: i32,
    #[allow(dead_code)]
    pub value2: i32,
}

#[test]
fn new() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let _ = BufferVk::new::<Data>(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::COLOR_BUFFER)
            .set_size(64),
    );
}

#[test]
fn new_as_array() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let _ = BufferVk::new_as_array::<u32>(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::COLOR_BUFFER)
            .set_size(std::mem::size_of::<u32>() * 64),
    );
}

#[test]
fn map() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let buffer = BufferVk::new::<Data>(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::COLOR_BUFFER)
            .set_size(std::mem::size_of::<Data>()),
    );
    buffer.map(|_: &Data| {});
}

#[test]
fn map_mut() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let buffer = BufferVk::new::<Data>(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::COLOR_BUFFER)
            .set_size(std::mem::size_of::<Data>()),
    );
    buffer.map_mut(|x: &mut Data| {
        x.value0 = 0;
        x.value1 = 1;
        x.value2 = 2;
    });
}

#[test]
fn map_as_array() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let buffer = BufferVk::new_as_array::<u32>(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::COLOR_BUFFER)
            .set_size(std::mem::size_of::<u32>() * 4),
    );
    buffer.map_as_array(|x: &[u32]| {
        let _ = x[0];
        let _ = x[1];
        let _ = x[2];
        let _ = x[3];
    });
}

#[test]
fn map_as_array_mut() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let buffer = BufferVk::new_as_array::<u32>(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::COLOR_BUFFER)
            .set_size(std::mem::size_of::<u32>() * 4),
    );
    buffer.map_as_array_mut(|x: &mut [u32]| {
        x[0] = 4;
        x[1] = 5;
        x[2] = 6;
        x[3] = 7;
    });
}
