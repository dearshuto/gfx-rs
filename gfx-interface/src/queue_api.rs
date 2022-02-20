use crate::IDevice;

pub struct QueueInfo {}

impl QueueInfo {
    pub fn new() -> QueueInfo {
        QueueInfo {}
    }
}

pub trait IQueue {
    fn new<TDevice: IDevice>(device: &TDevice, info: &QueueInfo) -> Self;
}
