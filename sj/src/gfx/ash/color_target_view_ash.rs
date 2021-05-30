use ash::version::DeviceV1_0;

use super::super::color_target_view_api::{ColorTargetViewInfo, IColorTargetViewImpl};
use super::super::{Device, Texture};

pub struct ColorTargetViewImpl<'a> {
    _texture: &'a Texture<'a>,
    _image_view: ash::vk::ImageView,
}

impl<'a> IColorTargetViewImpl<'a> for ColorTargetViewImpl<'a> {
    fn new(device: &Device, info: &'a ColorTargetViewInfo) -> Self {
        let device_ash = device.to_data().get_device();
        let texture = info.get_texture();

        unsafe {
            let image_view_create_info = ash::vk::ImageViewCreateInfo::builder()
                .image(*texture.to_data().get_image())
                .view_type(info.get_image_view_type_as_ash())
                .format(info.get_image_format_as_ash())
                //.components()
                .build();
            let image_view = device_ash
                .create_image_view(&image_view_create_info, None)
                .unwrap();

            Self {
                _texture: texture,
                _image_view: image_view,
            }
        }
    }
}

impl<'a> ColorTargetViewInfo<'a> {
    pub fn get_image_view_type_as_ash(&self) -> ash::vk::ImageViewType {
        ash::vk::ImageViewType::TYPE_2D
    }

    pub fn get_image_format_as_ash(&self) -> ash::vk::Format {
        match self.get_image_format() {
            &crate::gfx::ImageFormat::R8G8B8A8Unorm => ash::vk::Format::R8G8B8_UNORM,
        }
    }
}
