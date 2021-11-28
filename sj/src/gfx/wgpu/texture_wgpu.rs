use super::super::texture_api::{ITexture, TextureInfo};
use super::super::{Device, MemoryPool};
use crate::gfx::{GpuAccess, ImageFormat};

pub struct TextureWgpu<'a> {
    _device: &'a Device,
    _texture: wgpu::Texture,
}

impl<'a> TextureWgpu<'a> {
    pub fn get_texture(&self) -> &wgpu::Texture {
        &self._texture
    }
}

impl<'a> ITexture<'a> for TextureWgpu<'a> {
    fn calculate_required_size(_device: &Device, _info: &TextureInfo) -> u64 {
        0
    }

    fn calculate_required_alignment(_device: &Device, _info: &TextureInfo) -> u64 {
        1
    }

    fn new(
        device: &'a Device,
        info: &TextureInfo,
        _memory_pool: &MemoryPool,
        _offset: i64,
        _size: u64,
    ) -> Self {
        let texture_size = wgpu::Extent3d {
            width: info.get_width() as u32,
            height: info.get_height() as u32,
            depth_or_array_layers: info.get_depth() as u32,
        };

        let device_wgpu = device.to_data().get_device();
        let texture_wgpu = device_wgpu.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: info.get_image_format().to_wgpu(),
            usage: info.get_gpu_access_flags().to_wgpu(),
        });

        Self {
            _device: device,
            _texture: texture_wgpu,
        }
    }
}

impl GpuAccess {
    pub fn to_wgpu(&self) -> wgpu::TextureUsages {
        let mut result = wgpu::TextureUsages::empty();

        if self.contains(GpuAccess::TEXTURE) {
            result |= wgpu::TextureUsages::TEXTURE_BINDING;
        }
        if self.contains(GpuAccess::IMAGE) {
            result |= wgpu::TextureUsages::STORAGE_BINDING;
        }
        if self.contains(GpuAccess::COLOR_BUFFER) || self.contains(GpuAccess::DEPTH_STENCIL) {
            result |= wgpu::TextureUsages::RENDER_ATTACHMENT;
        }
        if self.contains(GpuAccess::READ) {
            result |= wgpu::TextureUsages::COPY_SRC;
        }
        if self.contains(GpuAccess::WRITE) {
            result |= wgpu::TextureUsages::COPY_DST;
        }

        result
    }
}

impl ImageFormat {
    pub fn to_wgpu(&self) -> wgpu::TextureFormat {
        match &self {
            ImageFormat::R8G8B8A8Unorm => wgpu::TextureFormat::Rgba8Unorm,
            ImageFormat::D32 => wgpu::TextureFormat::Depth32Float,
        }
    }
}
