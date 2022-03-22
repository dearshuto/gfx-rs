use sjgfx_interface::{GpuAccess, ITexture, TextureInfo};

use crate::{util, DeviceAsh};

pub struct TextureAsh {
    device: ash::Device,
    texture_handle: ash::vk::Image,
}

impl TextureAsh {
    fn create_image(device: &ash::Device, info: &TextureInfo) -> ash::vk::Image {
        let format = util::convert_image_format(info.get_image_format().clone());
        let usage = Self::convert_usage(info.get_gpu_access_flags().clone());

        let image_create_info = ash::vk::ImageCreateInfo::builder()
            .image_type(ash::vk::ImageType::TYPE_2D)
            .format(format)
            .sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            .extent(ash::vk::Extent3D {
                width: info.get_width() as u32,
                height: info.get_height() as u32,
                depth: info.get_depth() as u32,
            })
            .initial_layout(ash::vk::ImageLayout::UNDEFINED)
            .mip_levels(1)
            .array_layers(1)
            .samples(ash::vk::SampleCountFlags::TYPE_1)
            .tiling(ash::vk::ImageTiling::OPTIMAL)
            .usage(usage)
            .build();
        unsafe { device.create_image(&image_create_info, None).unwrap() }
    }

    fn convert_usage(gpu_access: GpuAccess) -> ash::vk::ImageUsageFlags {
        let mut result = ash::vk::ImageUsageFlags::empty();

        if gpu_access.contains(GpuAccess::TEXTURE) {
            result |= ash::vk::ImageUsageFlags::TRANSFER_DST;
            result |= ash::vk::ImageUsageFlags::SAMPLED;
        }
        if gpu_access.contains(GpuAccess::IMAGE) {
            // TODO
            //result |= ash::vk::ImageUsageFlags::STORAGE;
        }
        if gpu_access.contains(GpuAccess::COLOR_BUFFER) {
            result |= ash::vk::ImageUsageFlags::COLOR_ATTACHMENT
        }
        if gpu_access.contains(GpuAccess::DEPTH_STENCIL) {
            result |= ash::vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
        }
        if gpu_access.contains(GpuAccess::READ) {
            result |= ash::vk::ImageUsageFlags::TRANSFER_SRC;
        }
        if gpu_access.contains(GpuAccess::WRITE) {
            result |= ash::vk::ImageUsageFlags::TRANSFER_DST;
        }

        result
    }
}

impl ITexture for TextureAsh {
    type DeviceType = DeviceAsh;

    fn new(device: &Self::DeviceType, info: &sjgfx_interface::TextureInfo) -> Self {
        Self {
            device: device.get_device(),
            texture_handle: Self::create_image(device.get_device_ref(), info),
        }
    }
}

impl Drop for TextureAsh {
    fn drop(&mut self) {
        unsafe { self.device.destroy_image(self.texture_handle, None) }
    }
}
