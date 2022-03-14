use winit::event_loop::EventLoop;

pub struct DeviceInfo;
impl DeviceInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait IDevice {
    fn new(info: &DeviceInfo) -> Self;

    fn new_with_surface<TWindow>(info: &DeviceInfo, window: &TWindow, event_loop: &EventLoop<()>) -> Self
    where
        TWindow: raw_window_handle::HasRawWindowHandle;
}
