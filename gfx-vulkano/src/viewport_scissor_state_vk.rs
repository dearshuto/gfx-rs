use sjgfx_interface::ViewportScissorStateInfo;
use vulkano::pipeline::graphics::viewport::Viewport;

use crate::DeviceVk;

pub struct ViewportScissorStateVk {
    viewport: Viewport,
}

impl ViewportScissorStateVk {
    pub fn new(_device: &DeviceVk, _info: &ViewportScissorStateInfo) -> Self {
        Self {
            viewport: Viewport {
                origin: [0.0, 0.0],
                dimensions: [0.0, 0.0],
                depth_range: 0.0..1.0,
            },
        }
    }

    pub fn get_viewport(&self) -> &Viewport {
        &self.viewport
    }
}
