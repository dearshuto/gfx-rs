use std::sync::{Arc, Mutex};

use sjgfx_interface::{ISwapChain, SwapChainInfo};
use sjvi::IDisplayEventListener;
use wgpu::{SurfaceTexture, TextureFormat};

use crate::{ColorTargetViewWgpu, DeviceWgpu, FenceWgpu, SemaphoreWgpu};

pub struct SwapChainWgpu {
    surface: Arc<wgpu::Surface>,
    texture_format: TextureFormat,
    next_surface_texture: Option<Arc<Mutex<Option<SurfaceTexture>>>>,
}

impl SwapChainWgpu {
    pub fn new(device: &mut DeviceWgpu, info: &SwapChainInfo) -> Self {
        let adapter = device.get_adapter();
        let texture_format = device.get_surface().get_preferred_format(adapter).unwrap();

        device.update_surface_size(info.get_width(), info.get_height());
        Self {
            surface: device.clone_surface(),
            texture_format,
            next_surface_texture: None,
        }
    }

    pub fn acquire_next_scan_buffer_view(
        &mut self,
        _semaphore: Option<&mut SemaphoreWgpu>,
        _fence: Option<&mut FenceWgpu>,
    ) -> ColorTargetViewWgpu {
        let surface_texture = self.surface.get_current_texture().unwrap();
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

impl ISwapChain for SwapChainWgpu {
    type ColorTargetViewType = ColorTargetViewWgpu;
    type DeviceType = DeviceWgpu;
    type SemaphoreType = SemaphoreWgpu;
    type FenceType = FenceWgpu;

    fn new(device: &mut Self::DeviceType, info: &SwapChainInfo) -> Self {
        Self::new(device, info)
    }

    fn acquire_next_scan_buffer_view(
        &mut self,
        semaphore: Option<&mut Self::SemaphoreType>,
        fence: Option<&mut Self::FenceType>,
    ) -> Self::ColorTargetViewType {
        self.acquire_next_scan_buffer_view(semaphore, fence)
    }
}

impl IDisplayEventListener for SwapChainWgpu {
    fn on_resized(&mut self, _width: u32, _height: u32) {}
}
