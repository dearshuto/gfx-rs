use std::sync::Arc;

use sjgfx_interface::{ITexture, TextureInfo};
use vulkano::{
    format::Format,
    image::{view::ImageView, AttachmentImage, ImageUsage, ImmutableImage},
};

use crate::DeviceVk;

pub struct TextureVk {
    image_view: Option<Arc<ImageView<AttachmentImage>>>,
    _immutable_image_view: Option<Arc<ImageView<ImmutableImage>>>,
}

impl TextureVk {
    pub fn new(device: &DeviceVk, info: &TextureInfo) -> Self {
        let attach_usage = ImageUsage {
            transient_attachment: true,
            input_attachment: true,
            ..ImageUsage::none()
        };
        let dimensions = [info.get_width() as u32, info.get_height() as u32];
        let image_view = ImageView::new(
            AttachmentImage::with_usage(
                device.clone_device(),
                dimensions,
                Format::R8G8B8A8_UNORM,
                attach_usage,
            )
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
}
