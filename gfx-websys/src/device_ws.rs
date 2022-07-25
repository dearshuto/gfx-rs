use std::sync::Arc;

use sjgfx_interface::{DeviceInfo, IDevice};
use web_sys::WebGl2RenderingContext;

pub struct DeviceWsys {
    context: Arc<WebGl2RenderingContext>,
}

impl DeviceWsys {
    pub fn clone_context(&self) -> Arc<WebGl2RenderingContext> {
        self.context.clone()
    }
}

impl IDevice for DeviceWsys {
    type Display = sjvi::web_sys::Display;

    fn new(_info: &DeviceInfo) -> Self {
        todo!()
    }

    fn new_with_surface(_info: &DeviceInfo, display: &Self::Display) -> Self {
        Self {
            context: display.context.clone(),
        }
    }
}
