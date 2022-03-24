use crate::{IDevice, ITexture, ImageFormat};

pub struct TextureViewInfo {
    image_format: ImageFormat,
}

impl TextureViewInfo {
    pub fn new() -> Self {
        Self {
            image_format: ImageFormat::R8G8B8A8Unorm,
        }
    }

    pub fn get_format(&self) -> ImageFormat {
        self.image_format.clone()
    }

    pub fn set_format(mut self, image_format: ImageFormat) -> Self {
        self.image_format = image_format;
        self
    }
}

pub trait ITextureView {
    type DeviceType: IDevice;
    type TextureType: ITexture;

    fn new(device: &Self::DeviceType, info: &TextureViewInfo, texture: &Self::TextureType) -> Self;
}
