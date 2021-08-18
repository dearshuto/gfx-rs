use crate::gfx::{Device, DeviceInfo, Queue, QueueInfo};

#[test]
fn initialize() {
    let device = Device::new(&DeviceInfo::new());
    let _queue = Queue::new(&device, &QueueInfo::new());
}
