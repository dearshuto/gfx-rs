use std::sync::Arc;

use sjgfx_interface::{GpuAccess, ITexture, ImageFormat, TextureInfo};
use wgpu::util::DeviceExt;

use crate::DeviceWgpu;

pub struct TextureWgpu {
    texture: Arc<wgpu::Texture>,
}

impl TextureWgpu {
    pub fn new(device: &DeviceWgpu, info: &TextureInfo) -> Self {
        let texture_descriptor = Self::create_descriptor(info);
        let texture = device.get_device().create_texture(&texture_descriptor);

        Self {
            texture: Arc::new(texture),
        }
    }

    pub fn new_with_data(device: &DeviceWgpu, info: &TextureInfo, data: &[u8]) -> Self {
        let queue = device.get_queue();
        let texture_descriptor = Self::create_descriptor(info);
        let texture =
            device
                .get_device()
                .create_texture_with_data(queue, &texture_descriptor, data);

        Self {
            texture: Arc::new(texture),
        }
    }

    pub fn get_texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    pub fn close_texture(&self) -> Arc<wgpu::Texture> {
        self.texture.clone()
    }

    fn create_descriptor(info: &TextureInfo) -> wgpu::TextureDescriptor {
        let texture_size = wgpu::Extent3d {
            width: info.get_width() as u32,
            height: info.get_height() as u32,
            depth_or_array_layers: info.get_depth() as u32,
        };
        wgpu::TextureDescriptor {
            label: None,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::convert_format(info.get_image_format().clone()),
            usage: Self::convert_usage(info.get_gpu_access_flags()),
        }
    }

    fn convert_usage(gpu_access: &GpuAccess) -> wgpu::TextureUsages {
        let mut result = wgpu::TextureUsages::empty();

        if gpu_access.contains(GpuAccess::TEXTURE) {
            result |= wgpu::TextureUsages::TEXTURE_BINDING;
        }
        if gpu_access.contains(GpuAccess::IMAGE) {
            result |= wgpu::TextureUsages::STORAGE_BINDING;
        }
        if gpu_access.contains(GpuAccess::COLOR_BUFFER)
            || gpu_access.contains(GpuAccess::DEPTH_STENCIL)
        {
            result |= wgpu::TextureUsages::RENDER_ATTACHMENT;
        }
        if gpu_access.contains(GpuAccess::READ) {
            result |= wgpu::TextureUsages::COPY_SRC;
        }
        if gpu_access.contains(GpuAccess::WRITE) {
            result |= wgpu::TextureUsages::COPY_DST;
        }

        result
    }

    fn convert_format(format: ImageFormat) -> wgpu::TextureFormat {
        match format {
            ImageFormat::R8Unorm => wgpu::TextureFormat::R8Unorm,
            ImageFormat::R8G8B8Unorm => wgpu::TextureFormat::Rgba8Unorm,
            ImageFormat::R8G8B8A8Unorm => wgpu::TextureFormat::Rgba8Unorm,
            ImageFormat::D32 => wgpu::TextureFormat::Depth32Float,
        }
    }
}

impl ITexture for TextureWgpu {
    type DeviceType = DeviceWgpu;

    fn new(device: &Self::DeviceType, info: &TextureInfo) -> Self {
        Self::new(device, info)
    }
}
