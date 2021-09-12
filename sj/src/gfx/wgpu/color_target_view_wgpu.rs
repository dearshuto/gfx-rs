use super::super::color_target_view_api::IColorTargetViewImpl;
use super::super::Device;
use std::sync::Arc;

pub struct ColorTargetViewWgpu<'a> {
    _device: &'a Device,
    _texture_view: Arc<wgpu::TextureView>,
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
            _texture_view: Arc::new(texture_view),
        }
    }
}

impl<'a> ColorTargetViewWgpu<'a> {	
    pub fn get_texture_view(&self) -> &wgpu::TextureView {
        &self._texture_view
    }

	pub fn clone_texture_view(&self) -> Arc<wgpu::TextureView> {
		self._texture_view.clone()
	}
}
