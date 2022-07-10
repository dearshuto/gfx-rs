use std::sync::Arc;

use sjgfx_interface::{GpuAccess, ITexture, TextureInfo};
use wgpu::util::DeviceExt;

use crate::{util, DeviceWgpu};

pub struct TextureWgpu {
    texture: Arc<wgpu::Texture>,
}

impl TextureWgpu {
    pub fn new(device: &DeviceWgpu, info: &TextureInfo) -> Self {
        let texture_descriptor = Self::create_descriptor(info);
        let texture = device.get_device().create_texture(&texture_descriptor);

        Self {
            texture: Arc::new(texture),
        }
    }

    pub fn new_with_data(device: &DeviceWgpu, info: &TextureInfo, data: &[u8]) -> Self {
        let queue = device.get_queue();
        let texture_descriptor = Self::create_descriptor(info);
        let texture =
            device
                .get_device()
                .create_texture_with_data(queue, &texture_descriptor, data);

        Self {
            texture: Arc::new(texture),
        }
    }

    pub fn get_texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    pub fn close_texture(&self) -> Arc<wgpu::Texture> {
        self.texture.clone()
    }

    fn create_descriptor(info: &TextureInfo) -> wgpu::TextureDescriptor {
        let texture_size = wgpu::Extent3d {
            width: info.get_width() as u32,
            height: info.get_height() as u32,
            depth_or_array_layers: info.get_depth() as u32,
        };
        wgpu::TextureDescriptor {
            label: None,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: util::convert_format(info.get_image_format().clone()),
            usage: Self::convert_usage(info.get_gpu_access_flags()),
        }
    }

    fn convert_usage(gpu_access: &GpuAccess) -> wgpu::TextureUsages {
        let mut result = wgpu::TextureUsages::empty();

        if gpu_access.contains(GpuAccess::TEXTURE) {
            result |= wgpu::TextureUsages::TEXTURE_BINDING;
        }
        if gpu_access.contains(GpuAccess::IMAGE) {
            result |= wgpu::TextureUsages::STORAGE_BINDING;
        }
        if gpu_access.contains(GpuAccess::COLOR_BUFFER)
            || gpu_access.contains(GpuAccess::DEPTH_STENCIL)
        {
            result |= wgpu::TextureUsages::RENDER_ATTACHMENT;
        }
        if gpu_access.contains(GpuAccess::READ) {
            result |= wgpu::TextureUsages::COPY_SRC;
        }
        if gpu_access.contains(GpuAccess::WRITE) {
            result |= wgpu::TextureUsages::COPY_DST;
        }

        result
    }
}

impl ITexture for TextureWgpu {
    type DeviceType = DeviceWgpu;

    fn new(device: &mut Self::DeviceType, info: &TextureInfo) -> Self {
        Self::new(device, info)
    }

    fn new_with_data(device: &Self::DeviceType, info: &TextureInfo, data: &[u8]) -> Self {
        Self::new_with_data(device, info, data)
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{DebugMode, DeviceInfo, GpuAccess, IDevice, ImageFormat, TextureInfo};

    use crate::{DeviceWgpu, TextureWgpu};

    #[test]
    fn new_texture_r8_unorm() {
        new_impl(ImageFormat::R8Unorm, GpuAccess::TEXTURE);
    }

    #[test]
    fn new_texture_r8_uint() {
        new_impl(ImageFormat::R8Uint, GpuAccess::TEXTURE);
    }

    #[test]
    fn new_texture_r8_sint() {
        new_impl(ImageFormat::R8Sint, GpuAccess::TEXTURE);
    }

    #[test]
    fn new_texture_r8g8b8a8_unorm() {
        new_impl(ImageFormat::R8G8B8A8Unorm, GpuAccess::TEXTURE);
    }

    #[test]
    fn new_image_r8g8b8a8_unorm() {
        new_impl(ImageFormat::R8G8B8A8Unorm, GpuAccess::IMAGE);
    }

    #[test]
    fn new_image_r8_uint() {
        new_impl(ImageFormat::R8Uint, GpuAccess::IMAGE);
    }

    #[test]
    fn new_image_r8_unorm() {
        new_impl(ImageFormat::R8Unorm, GpuAccess::IMAGE);
    }

    fn new_impl(format: ImageFormat, gpu_access: GpuAccess) {
        let device = DeviceWgpu::new(&DeviceInfo::new().set_debug_mode(DebugMode::FullAssertion));
        let _ = TextureWgpu::new(
            &device,
            &TextureInfo::new()
                .set_width(64)
                .set_height(64)
                .set_image_format(format)
                .set_gpu_access_flags(gpu_access),
        );
    }
}
