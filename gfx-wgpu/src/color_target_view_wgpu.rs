use std::sync::{Arc, Mutex};

use sjgfx_interface::{ColorTargetViewInfo, IColorTargetView};
use wgpu::{TextureFormat, TextureViewDescriptor};

use crate::{util, DeviceWgpu, SwapChainWgpu, TextureWgpu};

#[derive(Debug, Clone)]
pub struct ColorTargetViewWgpu {
    _surface_texture: Option<Arc<Mutex<Option<wgpu::SurfaceTexture>>>>,
    texture_view: Option<Arc<wgpu::TextureView>>,
    texture_format: TextureFormat,
}

impl ColorTargetViewWgpu {
    pub fn new(_device: &DeviceWgpu, info: &ColorTargetViewInfo, texture: &TextureWgpu) -> Self {
        let view = texture
            .get_texture()
            .create_view(&TextureViewDescriptor::default());
        Self {
            _surface_texture: None,
            texture_view: Some(Arc::new(view)),
            texture_format: util::convert_format(info.get_image_format()),
        }
    }

    pub(crate) fn new_from_swap_chain(swap_chain: &SwapChainWgpu) -> Self {
        let surface_texture = swap_chain.clone_next_scan_buffer_surface_texture();
        let scan_buffer_view = surface_texture
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .texture
            .create_view(&TextureViewDescriptor::default());

        Self {
            _surface_texture: Some(surface_texture),
            texture_view: Some(Arc::new(scan_buffer_view)),
            texture_format: swap_chain.get_texture_format(),
        }
    }

    pub fn get_texture_view(&self) -> &wgpu::TextureView {
        self.texture_view.as_ref().unwrap()
    }

    pub fn get_texture_format(&self) -> wgpu::TextureFormat {
        self.texture_format
    }
}

impl IColorTargetView for ColorTargetViewWgpu {
    type DeviceType = DeviceWgpu;
    type TextureType = TextureWgpu;

    fn new(
        device: &Self::DeviceType,
        info: &sjgfx_interface::ColorTargetViewInfo,
        texture: &Self::TextureType,
    ) -> Self {
        Self::new(device, info, texture)
    }
}
