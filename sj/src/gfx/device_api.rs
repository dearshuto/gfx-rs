pub struct DeviceInfo {}

impl DeviceInfo {
    pub fn new() -> DeviceInfo {
        DeviceInfo {}
    }
}

pub trait TDeviceImpl {
    fn new(info: &DeviceInfo) -> Self;
}

pub struct TDeviceInterface<T>
where
    T: TDeviceImpl,
{
    device_impl: T,
}

impl<T: TDeviceImpl> TDeviceInterface<T> {
    pub fn new(info: &DeviceInfo) -> TDeviceInterface<T> {
        TDeviceInterface {
            device_impl: T::new(info),
        }
    }

    pub fn to_data(&mut self) -> &mut T {
        &mut self.device_impl
    }
}
