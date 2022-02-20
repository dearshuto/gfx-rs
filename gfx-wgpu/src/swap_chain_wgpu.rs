use std::sync::{Arc, Mutex};

use sjgfx_interface::SwapChainInfo;
use wgpu::{SurfaceTexture, TextureFormat};

use crate::{ColorTargetViewWgpu, DeviceWgpu};

pub struct SwapChainWgpu<'a> {
    device: &'a DeviceWgpu,
    texture_format: TextureFormat,
    next_surface_texture: Option<Arc<Mutex<Option<SurfaceTexture>>>>,
}

impl<'a> SwapChainWgpu<'a> {
    pub fn new(device: &'a DeviceWgpu, _info: &SwapChainInfo) -> Self {
        let adapter = device.get_adapter();
        let texture_format = device.get_surface().get_preferred_format(adapter).unwrap();
        Self {
            device,
            texture_format,
            next_surface_texture: None,
        }
    }

    pub fn acquire_next_scan_buffer_view(&mut self) -> ColorTargetViewWgpu {
        let surface_texture = self.device.get_surface().get_current_texture().unwrap();
        self.next_surface_texture = Some(Arc::new(Mutex::new(Some(surface_texture))));
        ColorTargetViewWgpu::new_from_swap_chain(self)
    }

    pub fn present(&mut self) {
        let mut temp = None;
        std::mem::swap(&mut temp, &mut self.next_surface_texture);

        let mut aa = None;
        std::mem::swap(&mut aa, &mut temp.unwrap().lock().unwrap());
        aa.unwrap().present();
    }

    pub fn get_texture_format(&self) -> TextureFormat {
        self.texture_format
    }

    pub fn clone_next_scan_buffer_surface_texture(&self) -> Arc<Mutex<Option<SurfaceTexture>>> {
        self.next_surface_texture.as_ref().unwrap().clone()
    }
}
