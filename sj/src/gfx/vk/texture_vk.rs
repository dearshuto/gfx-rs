use crate::gfx::texture_api::{ITexture, TextureInfo};
use crate::gfx::{Device, MemoryPool};

pub struct TextureVk {
    _immutable_image: std::sync::Arc<vulkano::image::ImmutableImage>,
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
        device: &'a Device,
        _info: &TextureInfo,
        _memory_pool: &MemoryPool,
        _offset: i64,
        _size: u64,
    ) -> Self {
        let queue = device.to_data().get_queue().clone();

        let mut image_data = Vec::<u8>::new();
        image_data.resize(4 * 640 * 480, 0);

        let (image, _future) = vulkano::image::ImmutableImage::from_iter(
            image_data.iter().cloned(),
            vulkano::image::ImageDimensions::Dim2d {
                width: 640,
                height: 480,
                array_layers: 1,
            },
            vulkano::image::MipmapsCount::One,
            vulkano::format::Format::R8G8B8A8Unorm,
            queue,
        )
        .unwrap();

        Self {
            _immutable_image: image,
        }
    }
}

impl TextureVk {
    pub fn clone_image_acess(&self) -> std::sync::Arc<dyn vulkano::image::ImageAccess> {
        self._immutable_image.clone()
    }
}
