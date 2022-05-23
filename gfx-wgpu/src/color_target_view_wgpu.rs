use std::sync::{Arc, Mutex, MutexGuard};

use sjgfx_interface::{ColorTargetViewInfo, IColorTargetView};
use wgpu::{TextureFormat, TextureViewDescriptor};

use crate::{swap_chain_wgpu::ScanBufferView, util, DeviceWgpu, TextureWgpu};

#[derive(Clone)]
pub struct ColorTargetViewWgpu {
    scan_buffer_view: Option<Arc<Mutex<Option<ScanBufferView>>>>,
    texture_format: TextureFormat,
}

impl ColorTargetViewWgpu {
    pub fn new(_device: &DeviceWgpu, info: &ColorTargetViewInfo, texture: &TextureWgpu) -> Self {
        let _view = texture
            .get_texture()
            .create_view(&TextureViewDescriptor::default());
        Self {
            scan_buffer_view: None,
            texture_format: util::convert_format(info.get_image_format()),
        }
    }

    pub(crate) fn new_from_scan_buffer_view(
        scan_buffer_view: Arc<Mutex<Option<ScanBufferView>>>,
        format: TextureFormat,
    ) -> Self {
        Self {
            scan_buffer_view: Some(scan_buffer_view),
            texture_format: format,
        }
    }

    pub fn get_texture_view(&self) -> MutexGuard<Option<ScanBufferView>> {
        self.scan_buffer_view.as_ref().unwrap().lock().unwrap()
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
