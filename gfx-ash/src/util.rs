use sjgfx_interface::ImageFormat;

pub fn convert_image_format(format: ImageFormat) -> ash::vk::Format {
    match format {
        ImageFormat::R8Sint => ash::vk::Format::R8_SINT,
        ImageFormat::R8Unorm => ash::vk::Format::R8_UNORM,
        ImageFormat::R8Snorm => ash::vk::Format::R8_SNORM,
        ImageFormat::R8Uint => ash::vk::Format::R8_UINT,
        ImageFormat::R32Uint => ash::vk::Format::R32_UINT,
        ImageFormat::R32Sint => ash::vk::Format::R32_SINT,
        ImageFormat::R8G8B8A8Sint => ash::vk::Format::R8G8B8A8_SINT,
        ImageFormat::R8G8B8A8Uint => ash::vk::Format::R8G8B8A8_UINT,
        ImageFormat::R8G8B8A8Unorm => ash::vk::Format::R8G8B8A8_UNORM,
        ImageFormat::R8G8B8Unorm => ash::vk::Format::R8G8B8_UNORM,
        ImageFormat::D32 => ash::vk::Format::D32_SFLOAT,
    }
}
