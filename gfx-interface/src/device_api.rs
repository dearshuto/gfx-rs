use crate::IDisplay;

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
    type Display: IDisplay;

    fn new(info: &DeviceInfo) -> Self;

    fn new_with_surface(info: &DeviceInfo, display: &Self::Display) -> Self;
}
