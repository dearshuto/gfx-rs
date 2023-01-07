use std::{ffi::CString, sync::Arc};

use glow::Context;
use glutin::{
    display::{Display, DisplayApiPreference},
    prelude::GlDisplay,
};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use sjgfx_interface::{DeviceInfo, IDevice};

pub struct DeviceGlow {
    context: Arc<glow::Context>,
}

impl DeviceGlow {
    pub fn new_from_handle<T>(_info: &DeviceInfo, handle: &T) -> Self
    where
        T: HasRawWindowHandle + HasRawDisplayHandle,
    {
        let preference = DisplayApiPreference::Cgl;
        let display = unsafe { Display::new(handle.raw_display_handle(), preference) }.unwrap();
        let context = unsafe {
            Context::from_loader_function(|symbol| {
                println!("{}", symbol);
                let symbol = CString::new(symbol).unwrap();
                let address = display.get_proc_address(symbol.as_c_str());
                println!("{}", address as i32);
                address
            })
        };
        // let surface = display
        //     .create_window_surface(config, surface_attributes)
        //     .unwrap();

        Self {
            context: Arc::new(context),
        }
    }

    pub fn clone_context(&self) -> Arc<glow::Context> {
        self.context.clone()
    }

    pub fn make_current(&mut self) {
        // ?
    }

    // 仮実装
    pub fn swap_buffers(&mut self) {
        // ?
        // if let Some(window) = &self.window {
        //     window.swap_buffers().unwrap();
        // }
    }
}

impl IDevice for DeviceGlow {
    type Display = sjvi::web_sys::Display;

    fn new(_info: &DeviceInfo) -> Self {
        // Self::new(info)
        todo!()
    }

    fn new_with_surface(_info: &DeviceInfo, _display: &Self::Display) -> Self {
        // Self::new_with_display(info, display)
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::DeviceInfo;

    use crate::DeviceGlow;

    #[test]
    fn new() {
        // let _device = DeviceGlow::new(&DeviceInfo::new());
    }
}
