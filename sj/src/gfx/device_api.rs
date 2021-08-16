use crate::vi::Layer;

pub struct DeviceInfo<'a> {
    _layer: Option<&'a Layer>,
}

impl<'a> DeviceInfo<'a> {
    pub fn new() -> DeviceInfo<'a> {
        DeviceInfo { _layer: None }
    }

    pub fn get_layer(&self) -> Option<&'a Layer> {
        self._layer
    }

    pub fn set_layer(mut self, layer: Option<&'a Layer>) -> DeviceInfo<'a> {
        self._layer = layer;
        self
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

    pub fn to_data(&self) -> &T {
        &self.device_impl
    }
}
