use crate::{DeviceWgpu, TextureWgpu};

pub struct DepthStencilViewWgpu<'a> {
    texture: &'a TextureWgpu,
    texture_view: wgpu::TextureView,
}

impl<'a> DepthStencilViewWgpu<'a> {
    pub fn new(_device: &DeviceWgpu, texture: &'a TextureWgpu) -> Self {
        let texture_view = texture
            .get_texture()
            .create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            texture_view,
        }
    }

    pub fn get_texture(&self) -> &TextureWgpu {
        self.texture
    }

    pub fn get_texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }
}