#[cfg(test)]
use crate::gfx::{CommandBuffer, CommandBufferInfo, Device, DeviceInfo, Queue, QueueInfo};

#[test]
fn initialize() {
    let device = Device::new(&DeviceInfo::new());
    let _queue = Queue::new(&device, &QueueInfo::new());
}

#[test]
fn sync_test() {
    let device = Device::new(&DeviceInfo::new());
    let mut queue = Queue::new(&device, &QueueInfo::new());
    queue.sync();
}

#[test]
fn execure_test() {
    let device = Device::new(&DeviceInfo::new());
    let mut command_buffer = CommandBuffer::new(&device, &CommandBufferInfo::new());
    let mut queue = Queue::new(&device, &QueueInfo::new());

    command_buffer.begin();
    command_buffer.end();
    queue.execute(&command_buffer);
    queue.sync();
}
