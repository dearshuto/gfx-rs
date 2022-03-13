pub struct DeviceInfo;
impl DeviceInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait IDevice {
    fn new(info: &DeviceInfo) -> Self;

    fn new_with_surface<TWindow>(info: &DeviceInfo, window: &TWindow) -> Self
    where
        TWindow: raw_window_handle::HasRawWindowHandle;
}
