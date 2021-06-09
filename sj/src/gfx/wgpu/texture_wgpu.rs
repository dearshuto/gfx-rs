use super::super::texture_api::{ITexture, TextureInfo};
use super::super::{Device, MemoryPool};

pub struct TextureWgpu {}

impl<'a> ITexture<'a> for TextureWgpu {
    fn calculate_required_size(device: &Device, info: &TextureInfo) -> u64 {
        todo!()
    }

    fn calculate_required_alignment(device: &Device, info: &TextureInfo) -> u64 {
        todo!()
    }

    fn new(
        device: &Device,
        info: &TextureInfo,
        memory_pool: &MemoryPool,
        offset: i64,
        size: u64,
    ) -> Self {
        todo!();
    }
}
