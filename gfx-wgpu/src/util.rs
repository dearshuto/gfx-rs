use sjgfx_interface::ImageFormat;

pub fn convert_format(format: ImageFormat) -> wgpu::TextureFormat {
    match format {
        ImageFormat::R8Unorm => wgpu::TextureFormat::R8Unorm,
        ImageFormat::R8Snorm => wgpu::TextureFormat::R8Snorm,
        ImageFormat::R8Uint => wgpu::TextureFormat::R8Uint,
        ImageFormat::R8Sint => wgpu::TextureFormat::R8Sint,
        ImageFormat::R32Uint => wgpu::TextureFormat::R32Uint,
        ImageFormat::R32Sint => wgpu::TextureFormat::R32Sint,
        ImageFormat::R8G8B8Unorm => wgpu::TextureFormat::Rgba8Unorm,
        ImageFormat::R8G8B8A8Unorm => wgpu::TextureFormat::Rgba8Unorm,
        ImageFormat::D32 => wgpu::TextureFormat::Depth32Float,
    }
}
