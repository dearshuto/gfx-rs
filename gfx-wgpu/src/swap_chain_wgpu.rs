use std::sync::{Arc, Mutex};

use sjgfx_interface::{ISwapChain, SwapChainInfo};
use wgpu::{SurfaceTexture, TextureFormat, TextureView, TextureViewDescriptor};

use crate::{ColorTargetViewWgpu, DeviceWgpu, FenceWgpu, SemaphoreWgpu};

pub struct ScanBufferView {
    pub surface_texture: SurfaceTexture,
    pub texture_view: TextureView,
}

pub struct SwapChainWgpu {
    device: Arc<wgpu::Device>,
    surface: Arc<wgpu::Surface>,
    adapter: Arc<wgpu::Adapter>,
    texture_format: TextureFormat,
    next_scan_buffer_view: Option<Arc<Mutex<Option<ScanBufferView>>>>,
    is: bool,
}

impl SwapChainWgpu {
    pub fn new(device: &mut DeviceWgpu, info: &SwapChainInfo) -> Self {
        let adapter = device.get_adapter();
        let texture_format = device.get_surface().get_preferred_format(adapter).unwrap();

        device.update_surface_size(info.get_width(), info.get_height());
        Self {
            device: device.close_device(),
            surface: device.clone_surface(),
            adapter: device.clone_adapter(),
            texture_format,
            next_scan_buffer_view: None,
            is: false
        }
    }

    pub fn acquire_next_scan_buffer_view(
        &mut self,
        _semaphore: Option<&mut SemaphoreWgpu>,
        _fence: Option<&mut FenceWgpu>,
    ) -> ColorTargetViewWgpu {

        if !self.is {
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: self.surface.get_preferred_format(&self.adapter).unwrap(),
                width: 1280,
                height: 960,
                present_mode: wgpu::PresentMode::Mailbox,
            };
            self.surface.configure(&self.device, &config);

            self.is = true;
        }

        let surface_texture = self.surface.get_current_texture().unwrap();
        let texture_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());
        let next_scan_buffer_view = ScanBufferView {
            surface_texture,
            texture_view,
        };
        let next_scan_buffer_view = Arc::new(Mutex::new(Some(next_scan_buffer_view)));
        self.next_scan_buffer_view = Some(next_scan_buffer_view.clone());
        ColorTargetViewWgpu::new_from_scan_buffer_view(next_scan_buffer_view, self.texture_format)
    }

    pub fn present(&mut self) {
        let mut temp = None;
        std::mem::swap(&mut temp, &mut self.next_scan_buffer_view);

        let mut aa = None;
        std::mem::swap(&mut aa, &mut temp.unwrap().lock().unwrap());
        aa.unwrap().surface_texture.present();
    }

    pub fn get_texture_format(&self) -> TextureFormat {
        self.texture_format
    }

    pub fn clone_next_scan_buffer_view(&self) -> Arc<Mutex<Option<ScanBufferView>>> {
        self.next_scan_buffer_view.as_ref().unwrap().clone()
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
