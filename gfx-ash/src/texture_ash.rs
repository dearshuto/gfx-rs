use sjgfx_interface::{GpuAccess, ITexture, TextureInfo};

use crate::{util, DeviceAsh, DeviceMemory};

pub struct TextureAsh {
    device: ash::Device,
    texture_handle: ash::vk::Image,
    image_view: ash::vk::ImageView,

    #[allow(dead_code)]
    device_memory: DeviceMemory,
}

impl TextureAsh {
    pub fn get_texture(&self) -> ash::vk::Image {
        self.texture_handle
    }

    pub fn get_image_view(&self) -> ash::vk::ImageView {
        self.image_view
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
        // テクスチャ
        let image = Self::create_image(device.get_device_ref(), info);

        // デバイスメモリ
        let device_memory = DeviceMemory::new(
            device,
            (info.get_width() * info.get_height() * 4 * 32) as usize,
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

        let format = util::convert_image_format(info.get_image_format().clone());
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
            texture_handle: image,
            image_view,
            device_memory,
        }
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
    use sjgfx_interface::{DeviceInfo, GpuAccess, ITexture, ImageFormat, TextureInfo};

    use crate::{DeviceAsh, TextureAsh};

    #[test]
    fn new_texture() {
        let device = DeviceAsh::new(&DeviceInfo::new());
        let _image_texture = TextureAsh::new(
            &device,
            &TextureInfo::new()
                .set_width(64)
                .set_height(64)
                .set_image_format(ImageFormat::R8Unorm)
                .set_gpu_access_flags(GpuAccess::TEXTURE),
        );
    }

    #[test]
    fn new_image() {
        let device = DeviceAsh::new(&DeviceInfo::new());
        let _image_texture = TextureAsh::new(
            &device,
            &TextureInfo::new()
                .set_width(64)
                .set_height(64)
                .set_image_format(ImageFormat::R8Unorm)
                .set_gpu_access_flags(GpuAccess::IMAGE),
        );
    }

    #[test]
    fn new_texture_and_image() {
        let device = DeviceAsh::new(&DeviceInfo::new());
        let _image_texture = TextureAsh::new(
            &device,
            &TextureInfo::new()
                .set_width(64)
                .set_height(64)
                .set_image_format(ImageFormat::R8Unorm)
                .set_gpu_access_flags(GpuAccess::IMAGE | GpuAccess::TEXTURE),
        );
    }
}
