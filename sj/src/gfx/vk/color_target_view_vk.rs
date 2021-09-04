use crate::gfx::color_target_view_api::{ColorTargetViewInfo, IColorTargetViewImpl};
use crate::gfx::Device;

pub struct ColorTargetViewVk<'a> {
    _device: &'a Device,
}

impl<'a> IColorTargetViewImpl<'a> for ColorTargetViewVk<'a> {
    fn new(device: &'a Device, info: &'a ColorTargetViewInfo) -> Self {
        todo!()
    }
}
