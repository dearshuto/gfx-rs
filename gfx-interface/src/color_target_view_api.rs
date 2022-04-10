use crate::{IDevice, ITexture, ImageFormat};

pub struct ColorTargetViewInfo {
    image_format: ImageFormat,
}

impl ColorTargetViewInfo {
    pub fn new() -> Self {
        Self {
            image_format: ImageFormat::R8G8B8A8Unorm,
        }
    }

    pub fn get_image_format(&self) -> ImageFormat {
        self.image_format.clone()
    }

    pub fn set_image_format(mut self, image_format: ImageFormat) -> Self {
        self.image_format = image_format;
        self
    }
}

pub trait IColorTargetView {
    type DeviceType: IDevice;
    type TextureType: ITexture;

    fn new(
        device: &Self::DeviceType,
        info: &ColorTargetViewInfo,
        texture: &Self::TextureType,
    ) -> Self;
}
