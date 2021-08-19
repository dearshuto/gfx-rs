use super::super::color_target_view_api::IColorTargetViewImpl;
use super::super::Device;

pub struct ColorTargetViewWgpu<'a> {
    _device: &'a Device,
    _texture_view: wgpu::TextureView,
}

impl<'a> IColorTargetViewImpl<'a> for ColorTargetViewWgpu<'a> {
    fn new(_device: &'a Device, _info: &'a crate::gfx::ColorTargetViewInfo) -> Self {
        todo!()
    }
}

impl<'a> ColorTargetViewWgpu<'a> {
    pub fn get_texture_view(&self) -> &wgpu::TextureView {
        &self._texture_view
    }
}
