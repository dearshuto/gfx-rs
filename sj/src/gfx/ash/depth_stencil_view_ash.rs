use ash::version::DeviceV1_0;

use super::super::depth_stencil_view_api::{DepthStencilViewInfo, IDepthStencilView};
use super::super::Device;

pub struct DepthStencilViewImpl<'a> {
    _device: &'a Device,
    _image_view: ash::vk::ImageView,
    _image: ash::vk::Image,
}

impl<'a> IDepthStencilView<'a> for DepthStencilViewImpl<'a> {
    fn new(device: &'a Device, info: &DepthStencilViewInfo) -> Self {
        let device_ash = device.to_data().get_device();

        let image_view_create_info = ash::vk::ImageViewCreateInfo::builder()
            .image(*info.get_texture().to_data().get_image())
            .view_type(ash::vk::ImageViewType::TYPE_2D)
            .format(ash::vk::Format::D32_SFLOAT)
            .subresource_range(
                ash::vk::ImageSubresourceRange::builder()
                    .aspect_mask(ash::vk::ImageAspectFlags::DEPTH)
                    .level_count(1)
                    .layer_count(1)
                    .build(),
            )
            .build();

        unsafe {
            let image_view = device_ash
                .create_image_view(&image_view_create_info, None)
                .unwrap();

            Self {
                _device: device,
                _image_view: image_view,
                _image: *info.get_texture().to_data().get_image(),
            }
        }
    }
}

impl<'a> DepthStencilViewImpl<'a> {
    pub fn get_image_view(&self) -> ash::vk::ImageView {
        self._image_view
    }

    pub fn get_image(&self) -> ash::vk::Image {
        self._image
    }
}

impl<'a> Drop for DepthStencilViewImpl<'a> {
    fn drop(&mut self) {
        let device_ash = self._device.to_data().get_device();

        unsafe {
            device_ash.destroy_image_view(self._image_view, None);
        }
    }
}
