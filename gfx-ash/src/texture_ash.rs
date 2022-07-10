use sjgfx_interface::{GpuAccess, ITexture, TextureInfo};

use crate::{util, DeviceAsh, DeviceMemory};

pub struct TextureAsh {
    device: ash::Device,
    texture_handle: ash::vk::Image,

    #[allow(dead_code)]
    device_memory: DeviceMemory,
}

impl TextureAsh {
    pub fn get_texture(&self) -> ash::vk::Image {
        self.texture_handle
    }

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
            result |= ash::vk::ImageUsageFlags::STORAGE;
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

    fn new(device: &mut Self::DeviceType, info: &sjgfx_interface::TextureInfo) -> Self {
        // テクスチャ
        let image = Self::create_image(device.get_device_ref(), info);

        let requirements = unsafe { device.get_device().get_image_memory_requirements(image) };

        // デバイスメモリ
        let device_memory = DeviceMemory::new(
            device,
            (info.get_width() * info.get_height() * 4 * 32) as usize,
            Some(requirements),
        );

        // デバイスメモリのひも付け
        unsafe {
            device.get_device_ref().bind_image_memory(
                image,
                device_memory.clone_device_memory_handle(),
                0,
            )
        }
        .unwrap();

        Self {
            device: device.get_device(),
            texture_handle: image,
            device_memory,
        }
    }

    fn new_with_data(_device: &Self::DeviceType, _info: &TextureInfo, _data: &[u8]) -> Self {
        todo!()
    }
}

impl Drop for TextureAsh {
    fn drop(&mut self) {
        //unsafe{ self.device.destroy_image_view(self.image_view, None) }
        unsafe { self.device.destroy_image(self.texture_handle, None) }
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{DebugMode, DeviceInfo, GpuAccess, ITexture, ImageFormat, TextureInfo};

    use crate::{DeviceAsh, TextureAsh};

    #[test]
    fn new_texture_r8_unorm() {
        new_impl(ImageFormat::R8Unorm, GpuAccess::TEXTURE);
    }

    #[test]
    fn new_texture_r8_sint() {
        new_impl(ImageFormat::R8Sint, GpuAccess::TEXTURE);
    }

    #[test]
    fn new_texture_r8_uint() {
        new_impl(ImageFormat::R8Uint, GpuAccess::TEXTURE);
    }

    #[test]
    fn new_image_r8_unorm() {
        new_impl(ImageFormat::R8Unorm, GpuAccess::IMAGE);
    }

    #[test]
    fn new_image_r8_uint() {
        new_impl(ImageFormat::R8Uint, GpuAccess::IMAGE);
    }

    #[test]
    fn new_image_r8_sint() {
        new_impl(ImageFormat::R8Sint, GpuAccess::IMAGE);
    }

    #[test]
    fn new_texture_and_image_r8_unorm() {
        new_impl(ImageFormat::R8Unorm, GpuAccess::TEXTURE | GpuAccess::IMAGE);
    }

    fn new_impl(image_format: ImageFormat, gpu_access: GpuAccess) {
        let mut device = DeviceAsh::new(&DeviceInfo::new().set_debug_mode(DebugMode::FullAssertion));
        let _ = TextureAsh::new(
            &mut device,
            &TextureInfo::new()
                .set_width(64)
                .set_height(64)
                .set_image_format(image_format)
                .set_gpu_access_flags(gpu_access),
        );
    }
}
