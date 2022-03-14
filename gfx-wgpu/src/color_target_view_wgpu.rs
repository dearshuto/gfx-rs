use std::sync::{Arc, Mutex};

use sjgfx_interface::IColorTargetView;
use wgpu::{TextureFormat, TextureViewDescriptor};

use crate::{DeviceWgpu, SwapChainWgpu};

pub struct ColorTargetViewWgpu {
    _surface_texture: Option<Arc<Mutex<Option<wgpu::SurfaceTexture>>>>,
    scan_buffer_view: Option<wgpu::TextureView>,
    texture_format: TextureFormat,
}

impl ColorTargetViewWgpu {
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
            scan_buffer_view: Some(scan_buffer_view),
            texture_format: swap_chain.get_texture_format(),
        }
    }

    pub fn get_texture_view(&self) -> &wgpu::TextureView {
        self.scan_buffer_view.as_ref().unwrap()
    }

    pub fn get_texture_format(&self) -> wgpu::TextureFormat {
        self.texture_format
    }
}

impl IColorTargetView for ColorTargetViewWgpu {
    type DeviceType = DeviceWgpu;

    fn new(_device: &Self::DeviceType, _info: &sjgfx_interface::ColorTargetViewInfo) -> Self {
        todo!()
    }
}
