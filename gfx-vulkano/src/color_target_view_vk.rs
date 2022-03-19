use std::sync::Arc;

use sjgfx_interface::{ColorTargetViewInfo, IColorTargetView, ImageFormat};
use vulkano::{
    format::Format,
    image::{view::ImageView, AttachmentImage, ImageViewAbstract},
};

use crate::{DeviceVk, SwapChainVk, TextureVk};

pub struct ColorTargetViewVk {
    texture: Option<Arc<ImageView<AttachmentImage>>>,
    scan_buffer_image_view: Option<Arc<dyn ImageViewAbstract>>,
    format: Format,
}

impl ColorTargetViewVk {
    pub fn new(_device: &DeviceVk, info: &ColorTargetViewInfo, texture: &TextureVk) -> Self {
        Self {
            texture: Some(texture.clone_attachment_image()),
            scan_buffer_image_view: None,
            format: Converter.convert_format(info.get_image_format()),
        }
    }

    pub fn new_from_swap_chain(swap_chain: &SwapChainVk) -> Self {
        let image_view = swap_chain.clone_current_image_view();
        Self {
            texture: None,
            scan_buffer_image_view: Some(image_view),
            format: swap_chain.get_swap_chain().format(),
        }
    }

    pub fn clone_image_view(&self) -> Arc<dyn ImageViewAbstract> {
        if let Some(texture) = &self.texture {
            texture.clone()
        } else if let Some(scan_buffer) = &self.scan_buffer_image_view {
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
        match image_format {
            ImageFormat::R8G8B8Unorm => Format::R8G8B8_UNORM,
            ImageFormat::R8G8B8A8Unorm => Format::R8G8B8A8_UNORM,
            ImageFormat::D32 => Format::D32_SFLOAT,
        }
    }
}

impl IColorTargetView for ColorTargetViewVk {
    type DeviceType = DeviceVk;

    fn new(_device: &Self::DeviceType, _info: &ColorTargetViewInfo) -> Self {
        todo!()
    }
}
