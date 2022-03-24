use std::sync::Arc;

use sjgfx_interface::{ITextureView, TextureViewInfo};

use crate::{DeviceWgpu, TextureWgpu};

pub struct TextureViewWgpu {
    texture_view: Arc<wgpu::TextureView>,
}

impl TextureViewWgpu {
    pub fn new(_device: &DeviceWgpu, _info: &TextureViewInfo, texture: &TextureWgpu) -> Self {
        let texture_view = texture
            .get_texture()
            .create_view(&wgpu::TextureViewDescriptor::default());
        Self {
            texture_view: Arc::new(texture_view),
        }
    }

    pub fn clone_texture_view(&self) -> Arc<wgpu::TextureView> {
        self.texture_view.clone()
    }
}

impl ITextureView for TextureViewWgpu {
    type DeviceType = DeviceWgpu;
    type TextureType = TextureWgpu;

    fn new(device: &Self::DeviceType, info: &TextureViewInfo, texture: &Self::TextureType) -> Self {
        Self::new(device, info, texture)
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{DeviceInfo, GpuAccess, IDevice, TextureInfo, TextureViewInfo};

    use crate::{DeviceWgpu, TextureViewWgpu, TextureWgpu};

    #[test]
    fn new() {
        let device = DeviceWgpu::new(&DeviceInfo::new());
        let texture = TextureWgpu::new(
            &device,
            &TextureInfo::new()
                .set_width(64)
                .set_height(64)
                .set_gpu_access_flags(GpuAccess::TEXTURE)
                .set_image_format(sjgfx_interface::ImageFormat::R8G8B8A8Unorm),
        );
        let _texture_view = TextureViewWgpu::new(&device, &TextureViewInfo::new(), &texture);
    }
}
