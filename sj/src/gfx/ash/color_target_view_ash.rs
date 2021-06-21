use ash::version::DeviceV1_0;

use super::super::color_target_view_api::{ColorTargetViewInfo, IColorTargetViewImpl};
use super::super::{Device, Texture};

pub struct ColorTargetViewImpl<'a> {
    _device: &'a Device,
    _texture: TextureData<'a>,
    _image_view: ash::vk::ImageView,
    _format: ash::vk::Format,
}

enum TextureData<'a> {
    Reference(&'a Texture<'a>),
    SwapChain(ash::vk::Image, i32, i32),
}

impl<'a> TextureData<'a> {
    pub fn get_image(&self) -> ash::vk::Image {
        match *self {
            Self::Reference(ref data) => *data.to_data().get_image(),
            Self::SwapChain(ref image, _width, _height) => *image,
        }
    }

    pub fn get_width(&self) -> i32 {
        match *self {
            Self::Reference(ref data) => data.to_data().get_width(),
            Self::SwapChain(ref _image, width, _height) => width,
        }
    }

    pub fn get_height(&self) -> i32 {
        match *self {
            Self::Reference(ref data) => data.to_data().get_height(),
            Self::SwapChain(ref _image, _width, height) => height,
        }
    }
}

impl<'a> IColorTargetViewImpl<'a> for ColorTargetViewImpl<'a> {
    fn new(device: &'a Device, info: &'a ColorTargetViewInfo) -> Self {
        let device_ash = device.to_data().get_device();
        let texture = info.get_texture();

        unsafe {
            let image_view_create_info = ash::vk::ImageViewCreateInfo::builder()
                .image(*texture.to_data().get_image())
                .view_type(info.get_image_view_type_as_ash())
                .format(info.get_image_format().to_ash())
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
            let image_view = device_ash
                .create_image_view(&image_view_create_info, None)
                .unwrap();

            Self {
                _device: device,
                _texture: TextureData::Reference(texture),
                _image_view: image_view,
                _format: info.get_image_format().to_ash(),
            }
        }
    }
}

impl<'a> ColorTargetViewImpl<'a> {
    pub fn new(
        device: &'a Device,
        texture: ash::vk::Image,
        width: i32,
        height: i32,
        image_view: ash::vk::ImageView,
        format: ash::vk::Format,
    ) -> Self {
        Self {
            _device: device,
            _texture: TextureData::SwapChain(texture, width, height),
            _image_view: image_view,
            _format: format,
        }
    }

    pub fn get_format(&self) -> ash::vk::Format {
        self._format
    }

    pub fn get_image_view(&self) -> &ash::vk::ImageView {
        &self._image_view
    }

    pub fn get_image(&self) -> ash::vk::Image {
        self._texture.get_image()
    }

    pub fn get_width(&self) -> i32 {
        self._texture.get_width()
    }

    pub fn get_height(&self) -> i32 {
        self._texture.get_height()
    }
}

impl<'a> ColorTargetViewInfo<'a> {
    pub fn get_image_view_type_as_ash(&self) -> ash::vk::ImageViewType {
        ash::vk::ImageViewType::TYPE_2D
    }
}

impl<'a> Drop for ColorTargetViewImpl<'a> {
    fn drop(&mut self) {
        let device_ash = self._device.to_data().get_device();

        unsafe {
            device_ash.destroy_image_view(self._image_view, None);
        }
    }
}
