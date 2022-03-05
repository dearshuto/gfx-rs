use crate::{DeviceWgpu, TextureWgpu};

pub struct TextureViewWgpu{
    texture_view: wgpu::TextureView,
}

impl TextureViewWgpu {
    pub fn new(_device: &DeviceWgpu, texture: &TextureWgpu) -> Self {
        let texture_view = texture.get_texture().create_view(&wgpu::TextureViewDescriptor::default());
        Self { texture_view }
    }

    pub fn get_texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }
}
