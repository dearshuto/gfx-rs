use std::sync::Arc;

use sjgfx_interface::{ITexture, TextureInfo};
use vulkano::{
    format::Format,
    image::{view::ImageView, AttachmentImage, ImmutableImage},
};

use crate::DeviceVk;

pub struct TextureVk {
    image_view: Option<Arc<ImageView<AttachmentImage>>>,
    _immutable_image_view: Option<Arc<ImageView<ImmutableImage>>>,
}

impl TextureVk {
    pub fn new(device: &DeviceVk, info: &TextureInfo) -> Self {
        let dimensions = [info.get_width() as u32, info.get_height() as u32];
        let image_view = ImageView::new_default(
            AttachmentImage::transient(device.clone_device(), dimensions, Format::D32_SFLOAT)
                .unwrap(),
        )
        .unwrap();

        Self {
            image_view: Some(image_view),
            _immutable_image_view: None,
        }
    }

    pub fn clone_attachment_image(&self) -> Arc<ImageView<AttachmentImage>> {
        self.image_view.as_ref().unwrap().clone()
    }
}

impl ITexture for TextureVk {
    type DeviceType = DeviceVk;

    fn new(device: &Self::DeviceType, info: &TextureInfo) -> Self {
        Self::new(device, info)
    }

    fn new_with_data(_device: &Self::DeviceType, _info: &TextureInfo, _data: &[u8]) -> Self {
        todo!()
    }
}
