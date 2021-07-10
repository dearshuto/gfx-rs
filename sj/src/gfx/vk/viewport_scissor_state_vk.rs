use crate::gfx::viewport_scissor_state_api::{
    IViewportScissorState, ScissorStateInfo, ViewportScissorStateInfo, ViewportStateInfo,
};
use crate::gfx::Device;

pub struct ViewportScissorStateVk {
    _viewport: Vec<vulkano::pipeline::viewport::Viewport>,
    _scissor: Vec<vulkano::pipeline::viewport::Scissor>,
}

impl<'a> IViewportScissorState<'a> for ViewportScissorStateVk {
    fn new(_device: &'a Device, info: &ViewportScissorStateInfo) -> Self {
        let viewports = info
            .get_viewport_state_info_array()
            .iter()
            .map(ViewportStateInfo::to_vk)
            .collect::<Vec<vulkano::pipeline::viewport::Viewport>>();
        let scissors = info
            .get_scissor_state_info_array()
            .iter()
            .map(ScissorStateInfo::to_vk)
            .collect::<Vec<vulkano::pipeline::viewport::Scissor>>();

        Self {
            _viewport: viewports,
            _scissor: scissors,
        }
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
