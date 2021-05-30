use ash::version::DeviceV1_0;

use super::super::texture_api::{ITexture, TextureInfo};
use super::super::{Device, MemoryPool, TextureUsage};

pub struct TextureImpl<'a> {
    _device: &'a Device,
    _image: ash::vk::Image,
}

impl<'a> TextureImpl<'a> {
    pub fn create_image(device: &Device, info: &TextureInfo) -> ash::vk::Image {
        let device_ash = device.to_data().get_device();

        let image_create_info = ash::vk::ImageCreateInfo::builder()
            .image_type(ash::vk::ImageType::TYPE_2D)
            .format(ash::vk::Format::R8G8B8A8_UNORM)
            .sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            .extent(ash::vk::Extent3D {
                width: info.get_width() as u32,
                height: info.get_height() as u32,
                depth: info.get_depth() as u32,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(ash::vk::SampleCountFlags::TYPE_1)
            .tiling(info.get_tiling_mode_as_ash())
            .usage(info.get_usage_as_ash())
            .build();
        unsafe { device_ash.create_image(&image_create_info, None).unwrap() }
    }
}

impl<'a> ITexture<'a> for TextureImpl<'a> {
    fn calculate_required_size(device: &Device, info: &TextureInfo) -> u64 {
        let device_ash = device.to_data().get_device();
        let image = TextureImpl::create_image(device, info);

        unsafe {
            // 必要な情報を取得したら即刻破棄
            let requirement = device_ash.get_image_memory_requirements(image);
            device_ash.destroy_image(image, None);

            requirement.size
        }
    }

    fn calculate_required_alignment(device: &Device, info: &TextureInfo) -> u64 {
        let device_ash = device.to_data().get_device();
        let image = TextureImpl::create_image(device, info);

        unsafe {
            // 必要な情報を取得したら即刻破棄
            let requirement = device_ash.get_image_memory_requirements(image);
            device_ash.destroy_image(image, None);

            requirement.alignment
        }
    }

    fn new(
        device: &'a Device,
        info: &TextureInfo,
        memory_pool: &MemoryPool,
        offset: i64,
        _size: u64,
    ) -> Self {
        let device_ash = device.to_data().get_device();
        let image = TextureImpl::create_image(device, info);

        unsafe {
            let device_memory = memory_pool.to_data().get_memory_pool();
            device_ash
                .bind_image_memory(image, device_memory, offset as u64)
                .unwrap();

            Self {
                _device: device,
                _image: image,
            }
        }
    }
}

impl<'a> Drop for TextureImpl<'a> {
    fn drop(&mut self) {
        let device_ash = self._device.to_data().get_device();
        unsafe {
            device_ash.destroy_image(self._image, None);
        }
    }
}

impl TextureInfo {
    pub fn get_tiling_mode_as_ash(&self) -> ash::vk::ImageTiling {
        ash::vk::ImageTiling::OPTIMAL
    }

    pub fn get_usage_as_ash(&self) -> ash::vk::ImageUsageFlags {
        let mut result = ash::vk::ImageUsageFlags::empty();

        if self.get_texture_usage().contains(TextureUsage::TEXTURE) {
            result |= ash::vk::ImageUsageFlags::TRANSFER_DST;
            result |= ash::vk::ImageUsageFlags::SAMPLED;
        }
        if self.get_texture_usage().contains(TextureUsage::IMAGE) {
            // TODO
            //result |= ash::vk::ImageUsageFlags::STORAGE;
        }

        result
    }
}