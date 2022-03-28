use winit::event_loop::EventLoop;

use crate::enums::DebugMode;

pub struct DeviceInfo {
    debug_mode: DebugMode,
}

impl DeviceInfo {
    pub fn new() -> Self {
        Self {
            debug_mode: DebugMode::Full,
        }
    }

    pub fn get_debug_mode(&self) -> DebugMode {
        self.debug_mode.clone()
    }

    pub fn set_debug_mode(mut self, debug_mode: DebugMode) -> Self {
        self.debug_mode = debug_mode;
        self
    }
}

pub trait IDevice {
    fn new(info: &DeviceInfo) -> Self;

    fn new_with_surface<TWindow>(
        info: &DeviceInfo,
        window: &TWindow,
        event_loop: &EventLoop<()>,
    ) -> Self
    where
        TWindow: raw_window_handle::HasRawWindowHandle;
}
