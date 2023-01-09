use crate::enums::DebugMode;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

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

    fn new_with_handle<T>(info: &DeviceInfo, raw_handle: &T) -> Self
    where
        T: HasRawWindowHandle + HasRawDisplayHandle;
}
