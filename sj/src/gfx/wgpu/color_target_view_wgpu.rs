use super::super::color_target_view_api::IColorTargetViewImpl;
use super::super::Device;

pub struct ColorTargetViewWgpu<'a> {
    _device: &'a Device,
}

impl<'a> IColorTargetViewImpl<'a> for ColorTargetViewWgpu<'a> {
    fn new(_device: &'a Device, _info: &'a crate::gfx::ColorTargetViewInfo) -> Self {
        todo!()
    }
}
