use super::super::color_target_view_api::IColorTargetViewImpl;
use super::super::Device;

pub struct ColorTargetViewWgpu<'a> {
    _device: &'a Device,
    _texture_view: wgpu::TextureView,
}

impl<'a> ColorTargetViewWgpu<'a> {
    pub fn new_internal(device: &'a Device, texture_view: wgpu::TextureView) -> Self {
        Self {
            _device: device,
            _texture_view: texture_view,
        }
    }

    pub fn get_texture_view(&self) -> &wgpu::TextureView {
        &self._texture_view
    }
}

impl<'a> IColorTargetViewImpl<'a> for ColorTargetViewWgpu<'a> {
    fn new(device: &'a Device, info: &'a crate::gfx::ColorTargetViewInfo) -> Self {
        let texture_view = info
            .get_texture()
            .to_data()
            .get_texture()
            .create_view(&wgpu::TextureViewDescriptor::default());
        Self {
            _device: device,
            _texture_view: texture_view,
        }
    }
}
