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
    fn new(_device: &'a crate::gfx::Device, _info: &crate::gfx::ViewportScissorStateInfo) -> Self {
        todo!()
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

