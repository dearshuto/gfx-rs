use super::super::texture_api::{ITexture, TextureInfo};
use super::super::{Device, MemoryPool};

pub struct TextureWgpu<'a> {
    _device: &'a Device,
    _texture: wgpu::Texture,
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
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_SRC,
        });

        Self {
            _device: device,
            _texture: texture_wgpu,
        }
    }
}
