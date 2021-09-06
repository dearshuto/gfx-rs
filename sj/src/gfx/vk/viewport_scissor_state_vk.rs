use crate::gfx::viewport_scissor_state_api::{
    IViewportScissorState, ScissorStateInfo, ViewportScissorStateInfo, ViewportStateInfo,
};
use crate::gfx::Device;

pub struct ViewportScissorStateVk {
    _viewport_state_info_array: Vec<ViewportStateInfo>,
    _scissor_state_info_array: Vec<ScissorStateInfo>,
}

impl<'a> IViewportScissorState<'a> for ViewportScissorStateVk {
    fn new(_device: &'a Device, info: &ViewportScissorStateInfo) -> Self {
        Self {
            _viewport_state_info_array: info.get_viewport_state_info_array().to_vec(),
            _scissor_state_info_array: info.get_scissor_state_info_array().to_vec(),
        }
    }
}

impl ViewportScissorStateVk {
    pub fn get_viewport_state_info_array(&self) -> &[ViewportStateInfo] {
        &self._viewport_state_info_array
    }

    pub fn get_scissor_state_info_array(&self) -> &[ScissorStateInfo] {
        &self._scissor_state_info_array
    }
}

impl ViewportStateInfo {
    pub fn to_vk(&self) -> vulkano::pipeline::viewport::Viewport {
        vulkano::pipeline::viewport::Viewport {
            origin: [self.get_origin_x(), self.get_origin_y()],
            dimensions: [self.get_width(), self.get_height()],
            depth_range: 0.0..1.0,
        }
    }
}

impl ScissorStateInfo {
    pub fn to_vk(&self) -> vulkano::pipeline::viewport::Scissor {
        vulkano::pipeline::viewport::Scissor {
            origin: [self.get_origin_x(), self.get_origin_y()],
            dimensions: [self.get_width() as u32, self.get_height() as u32],
        }
    }
}
