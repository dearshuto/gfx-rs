use sjgfx_interface::ImageFormat;
use vulkano::format::Format;

pub fn convert_format(image_format: ImageFormat) -> Format {
    match image_format {
        ImageFormat::R8Unorm => Format::R8_UNORM,
        ImageFormat::R8Snorm => Format::R8_SNORM,
        ImageFormat::R8Sint => Format::R8_SINT,
        ImageFormat::R8Uint => Format::R8_UINT,
        ImageFormat::R32Sint => Format::R32_SINT,
        ImageFormat::R32Uint => Format::R32_UINT,
        ImageFormat::R8G8B8A8Uint => Format::R8G8B8_UINT,
        ImageFormat::R8G8B8A8Sint => Format::R8G8B8_SINT,
        ImageFormat::R8G8B8Unorm => Format::R8G8B8_UNORM,
        ImageFormat::R8G8B8A8Unorm => Format::R8G8B8A8_UNORM,
        ImageFormat::D32 => Format::D32_SFLOAT,
    }
}
