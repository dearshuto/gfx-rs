fn main()
{
    let device_info = sj::gfx::DeviceInfo::new();
    let device = sj::gfx::Device::new(&device_info);

    let queue_info = sj::gfx::QueueInfo::new();
    let mut queue = sj::gfx::Queue::new(&device, &queue_info);
    queue.flush();
    queue.sync();
}
