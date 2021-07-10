use crate::gfx::texture_api::{ITexture, TextureInfo};
use crate::gfx::{Device, MemoryPool};

pub struct TextureVk {
    _immutable_image: Option<std::sync::Arc<vulkano::image::ImmutableImage>>,
}

impl<'a> ITexture<'a> for TextureVk {
    fn calculate_required_size(
        _device: &crate::gfx::Device,
        _info: &crate::gfx::TextureInfo,
    ) -> u64 {
        0
    }

    fn calculate_required_alignment(
        _device: &crate::gfx::Device,
        _info: &crate::gfx::TextureInfo,
    ) -> u64 {
        1
    }

    fn new(
        _device: &'a Device,
        _info: &TextureInfo,
        _memory_pool: &MemoryPool,
        _offset: i64,
        _size: u64,
    ) -> Self {
        Self {
            _immutable_image: None,
        }
    }
}
