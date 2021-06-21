use crate::gfx::ImageFormat;

impl ImageFormat {
    pub fn to_ash(&self) -> ash::vk::Format {
        match self {
            ImageFormat::R8G8B8A8Unorm => ash::vk::Format::R8G8B8A8_UNORM,
            ImageFormat::D32 => ash::vk::Format::D32_SFLOAT,
        }
    }
}
