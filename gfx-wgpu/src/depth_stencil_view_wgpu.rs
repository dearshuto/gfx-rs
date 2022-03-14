use std::sync::Arc;

use sjgfx_interface::IDepthStencilView;

use crate::{DeviceWgpu, TextureWgpu};

pub struct DepthStencilViewWgpu {
    texture: Arc<wgpu::Texture>,
    texture_view: Arc<wgpu::TextureView>,
}

impl DepthStencilViewWgpu {
    pub fn new(_device: &DeviceWgpu, texture: &TextureWgpu) -> Self {
        let texture = texture.close_texture();
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            texture_view: Arc::new(texture_view),
        }
    }

    pub fn get_texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    pub fn get_texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }

    pub fn close_texture_view(&self) -> Arc<wgpu::TextureView> {
        self.texture_view.clone()
    }
}

impl IDepthStencilView for DepthStencilViewWgpu {
    type DeviceType = DeviceWgpu;
    type TextureType = TextureWgpu;

    fn new(
        device: &Self::DeviceType,
        _info: &sjgfx_interface::DepthStencilStateInfo,
        texture: &Self::TextureType,
    ) -> Self {
        Self::new(device, texture)
    }
}
