use super::super::Device;

use super::super::viewport_scissor_state_api::{
    IViewportScissorState, ScissorStateInfo, ViewportScissorStateInfo, ViewportStateInfo,
};

pub struct ViewportScissorStateImpl {
    _viewport_state_info: Vec<ash::vk::Viewport>,
    _scissor_state_info: Vec<ash::vk::Rect2D>,
}

impl<'a> IViewportScissorState<'a> for ViewportScissorStateImpl {
    fn new(_device: &'a Device, info: &ViewportScissorStateInfo) -> Self {
        let viewport_state_info: Vec<ash::vk::Viewport> = info
            .get_viewport_state_info_array()
            .iter()
            .map(ViewportStateInfo::as_ash)
            .collect();
        let scissor_state_info: Vec<ash::vk::Rect2D> = info
            .get_scissor_state_info_array()
            .iter()
            .map(ScissorStateInfo::as_ash)
            .collect();

        Self {
            _viewport_state_info: viewport_state_info,
            _scissor_state_info: scissor_state_info,
        }
    }
}

impl ViewportScissorStateImpl {
    pub fn get_viewports(&self) -> &[ash::vk::Viewport] {
        &self._viewport_state_info
    }

    pub fn get_scissor_state(&self) -> &[ash::vk::Rect2D] {
        &self._scissor_state_info
    }
}

impl ViewportStateInfo {
    pub fn as_ash(&self) -> ash::vk::Viewport {
        ash::vk::Viewport::builder()
            .x(self.get_origin_x())
            .y(self.get_origin_y())
            .width(self.get_width())
            .height(self.get_height())
            .build()
    }
}

impl ScissorStateInfo {
    pub fn as_ash(&self) -> ash::vk::Rect2D {
        ash::vk::Rect2D::builder()
            .offset(
                ash::vk::Offset2D::builder()
                    .x(self.get_origin_x())
                    .y(self.get_origin_y())
                    .build(),
            )
            .extent(
                ash::vk::Extent2D::builder()
                    .width(self.get_width() as u32)
                    .height(self.get_height() as u32)
                    .build(),
            )
            .build()
    }
}
