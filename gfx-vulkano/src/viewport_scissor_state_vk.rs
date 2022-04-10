use sjgfx_interface::ViewportScissorStateInfo;
use vulkano::pipeline::graphics::viewport::{Scissor, Viewport};

use crate::DeviceVk;

pub struct ViewportScissorStateVk {
    viewports: Vec<Viewport>,
    scissors: Vec<Scissor>,
}

impl ViewportScissorStateVk {
    pub fn new(_device: &DeviceVk, info: &ViewportScissorStateInfo) -> Self {
        // OpenGL と同じビューポート座標になるようにしています。
        // Device の Extensions の設定によって変えてもいいかも
        let viewports = info
            .get_viewport_state_info_array()
            .iter()
            .map(|x| Viewport {
                origin: [x.get_origin_x(), x.get_height() + x.get_origin_y()],
                dimensions: [x.get_width(), -x.get_height()],
                depth_range: 0.0..1.0,
            })
            .collect::<Vec<_>>();

        let scissors = info
            .get_scissor_state_info_array()
            .iter()
            .map(|x| Scissor {
                origin: [x.get_origin_x() as u32, x.get_origin_y() as u32],
                dimensions: [x.get_width() as u32, x.get_height() as u32],
            })
            .collect::<Vec<_>>();

        Self {
            viewports,
            scissors,
        }
    }

    pub(crate) fn view(&self) -> ViewportScissorStateView {
        ViewportScissorStateView {
            viewports: self.viewports.to_vec(),
            scissors: self.scissors.to_vec(),
        }
    }
}

pub struct ViewportScissorStateView {
    pub viewports: Vec<Viewport>,
    pub scissors: Vec<Scissor>,
}
