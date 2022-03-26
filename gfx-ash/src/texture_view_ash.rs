use sjgfx_interface::{ITextureView, TextureViewInfo};

use crate::{util, DeviceAsh, TextureAsh};

pub struct TextureViewAsh {
    device: ash::Device,
    image_view: ash::vk::ImageView,
}

impl TextureViewAsh {
    pub fn new(device: &DeviceAsh, info: &TextureViewInfo, texture: &TextureAsh) -> Self {
        let image = texture.get_texture();
        let format = util::convert_image_format(info.get_format().clone());
        let create_info = ash::vk::ImageViewCreateInfo::builder()
            .image(image)
            .view_type(ash::vk::ImageViewType::TYPE_2D)
            .format(format)
            .components(
                ash::vk::ComponentMapping::builder()
                    .r(ash::vk::ComponentSwizzle::R)
                    .g(ash::vk::ComponentSwizzle::G)
                    .b(ash::vk::ComponentSwizzle::B)
                    .a(ash::vk::ComponentSwizzle::A)
                    .build(),
            )
            .subresource_range(
                ash::vk::ImageSubresourceRange::builder()
                    .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build(),
            )
            .build();
        let image_view =
            unsafe { device.get_device().create_image_view(&create_info, None) }.unwrap();

        Self {
            device: device.get_device(),
            image_view,
        }
    }

    pub fn get_image_view(&self) -> ash::vk::ImageView {
        self.image_view
    }
}

impl ITextureView for TextureViewAsh {
    type DeviceType = DeviceAsh;
    type TextureType = TextureAsh;

    fn new(device: &Self::DeviceType, info: &TextureViewInfo, texture: &Self::TextureType) -> Self {
        Self::new(device, info, texture)
    }
}

impl Drop for TextureViewAsh {
    fn drop(&mut self) {
        unsafe { self.device.destroy_image_view(self.image_view, None) }
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{
        DeviceInfo, GpuAccess, ITexture, ImageFormat, TextureInfo, TextureViewInfo,
    };

    use crate::{DeviceAsh, TextureAsh, TextureViewAsh};

    #[test]
    fn new_texture() {
        let device = DeviceAsh::new(&DeviceInfo::new());
        let texture = TextureAsh::new(
            &device,
            &TextureInfo::new()
                .set_width(64)
                .set_height(64)
                .set_image_format(ImageFormat::R8Unorm)
                .set_gpu_access_flags(GpuAccess::TEXTURE),
        );
        let _view = TextureViewAsh::new(
            &device,
            &TextureViewInfo::new().set_format(ImageFormat::R8Unorm),
            &texture,
        );
    }
}
