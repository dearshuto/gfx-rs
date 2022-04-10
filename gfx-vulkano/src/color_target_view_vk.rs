use std::sync::Arc;

use sjgfx_interface::{ColorTargetViewInfo, IColorTargetView, ImageFormat};
use vulkano::{
    format::Format,
    image::{view::ImageView, AttachmentImage, ImageViewAbstract},
};

use crate::{util, DeviceVk, SwapChainVk, TextureVk};

pub struct ColorTargetViewVk {
    texture: Option<Arc<ImageView<AttachmentImage>>>,
    image_view: Option<Arc<dyn ImageViewAbstract>>,
    format: Format,
}

impl ColorTargetViewVk {
    pub fn new(_device: &DeviceVk, info: &ColorTargetViewInfo, texture: &TextureVk) -> Self {
        let image_view = ImageView::new_default(texture.clone_image()).unwrap();
        Self {
            texture: Some(texture.clone_attachment_image()),
            image_view: Some(image_view),
            format: Converter.convert_format(info.get_image_format()),
        }
    }

    pub fn new_from_swap_chain(swap_chain: &SwapChainVk) -> Self {
        let image_view = swap_chain.clone_current_image_view();
        Self {
            texture: None,
            image_view: Some(image_view),
            format: swap_chain.get_swap_chain().image_format(),
        }
    }

    pub fn clone_image_view(&self) -> Arc<dyn ImageViewAbstract> {
        if let Some(texture) = &self.texture {
            texture.clone()
        } else if let Some(scan_buffer) = &self.image_view {
            scan_buffer.clone()
        } else {
            todo!()
        }
    }

    pub fn get_format(&self) -> Format {
        self.format
    }
}

struct Converter;
impl Converter {
    pub fn convert_format(&self, image_format: ImageFormat) -> Format {
        util::convert_format(image_format)
    }
}

impl IColorTargetView for ColorTargetViewVk {
    type DeviceType = DeviceVk;
    type TextureType = TextureVk;

    fn new(
        device: &Self::DeviceType,
        info: &ColorTargetViewInfo,
        texture: &Self::TextureType,
    ) -> Self {
        Self::new(device, info, texture)
    }
}
