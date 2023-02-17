use std::sync::{Arc, Mutex};

use sjgfx_interface::{IDisplayEventListener, ISwapChain, SwapChainInfo};
use wgpu::{CompositeAlphaMode, SurfaceTexture, TextureFormat};

use crate::{
    detail::SwapChainPipeline, ColorTargetViewWgpu, DeviceWgpu, FenceWgpu, QueueWgpu, SemaphoreWgpu,
};

pub struct SwapChainWgpu {
    device: Arc<wgpu::Device>,
    surface: Arc<wgpu::Surface>,
    texture_format: wgpu::TextureFormat,
    next_surface_texture: Option<Arc<Mutex<Option<SurfaceTexture>>>>,

    swap_chain_pipeline: SwapChainPipeline,
}

impl SwapChainWgpu {
    pub fn new(device: &mut DeviceWgpu, info: &SwapChainInfo) -> Self {
        let adapter = device.get_adapter();
        let swapchain_capabilities = device.get_surface().get_capabilities(&adapter);
        let texture_format = swapchain_capabilities.formats[0];
        let swap_chain_pipeline = SwapChainPipeline::new(device.close_device(), texture_format);

        let mut result = Self {
            device: device.close_device(),
            surface: device.clone_surface(),
            texture_format,
            next_surface_texture: None,
            swap_chain_pipeline,
        };

        result.on_resized(info.get_width(), info.get_height());
        result
    }

    pub fn acquire_next_scan_buffer_view(
        &mut self,
        _semaphore: Option<&mut SemaphoreWgpu>,
        _fence: Option<&mut FenceWgpu>,
    ) -> &mut ColorTargetViewWgpu {
        self.swap_chain_pipeline.get_color_target_view_mut()
    }

    pub fn present(&mut self, queue: &mut QueueWgpu) {
        // スキャンバッファのビューを作成
        let surface_texture = self.surface.get_current_texture().unwrap();
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // カラーターゲットの内容をスキャンバッファにコピー
        let command_buffer = self.swap_chain_pipeline.build_command(&texture_view);
        queue.submit_command_buffer_direct(command_buffer);
        surface_texture.present();
    }

    pub fn get_scan_buffer_view(&self) -> &ColorTargetViewWgpu {
        self.swap_chain_pipeline.get_color_target_view()
    }

    pub fn get_scan_buffer_view_mut(&mut self) -> &mut ColorTargetViewWgpu {
        self.swap_chain_pipeline.get_color_target_view_mut()
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
    ) -> &mut Self::ColorTargetViewType {
        self.acquire_next_scan_buffer_view(semaphore, fence)
    }
}

impl IDisplayEventListener for SwapChainWgpu {
    fn on_resized(&mut self, width: u32, height: u32) {
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.texture_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        self.surface.configure(&self.device, &config);
        self.swap_chain_pipeline.set_size(width, height);
    }
}
