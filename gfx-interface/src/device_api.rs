pub struct DeviceInfo;
impl DeviceInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait IDevice {
    fn new(info: &DeviceInfo) -> Self;
}
