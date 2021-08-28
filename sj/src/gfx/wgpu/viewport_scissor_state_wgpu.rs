use crate::gfx::ScissorStateInfo;

use super::super::viewport_scissor_state_api::IViewportScissorState;

#[derive(Debug, Copy, Clone)]
pub struct ViewportScissorStateWgpu {
    _origin_x: f32,
    _origin_y: f32,
    _width: f32,
    _height: f32,
    _scissor_state_info: ScissorStateInfo,
}

impl<'a> IViewportScissorState<'a> for ViewportScissorStateWgpu {
    fn new(_device: &'a crate::gfx::Device, info: &crate::gfx::ViewportScissorStateInfo) -> Self {
        Self {
            _origin_x: info.get_viewport_state_info_array()[0].get_origin_x(),
            _origin_y: info.get_viewport_state_info_array()[0].get_origin_y(),
            _width: info.get_viewport_state_info_array()[0].get_width(),
            _height: info.get_viewport_state_info_array()[0].get_height(),
            _scissor_state_info: info.get_scissor_state_info_array()[0],
        }
    }
}

impl ViewportScissorStateWgpu {
    pub fn get_origin_x(&self) -> f32 {
        self._origin_x
    }

    pub fn get_origin_y(&self) -> f32 {
        self._origin_y
    }

    pub fn push(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_viewport(
            self._origin_x,
            self._origin_y,
            self._width,
            self._height,
            0.0,
            1.0,
        );

        let scissor_state_info = &self._scissor_state_info;
        render_pass.set_scissor_rect(
            scissor_state_info.get_origin_x() as u32,
            scissor_state_info.get_height() as u32,
            scissor_state_info.get_width() as u32,
            scissor_state_info.get_height() as u32,
        );
    }
}
