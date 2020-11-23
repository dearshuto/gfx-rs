fn main()
{
    let device_info = sj::gfx::DeviceInfo::new();
    let mut device = sj::gfx::Device::new();
    device.Initialize(device_info);
}
