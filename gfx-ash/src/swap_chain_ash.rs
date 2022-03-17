use sjgfx_interface::{ISwapChain, SwapChainInfo};

use crate::{ColorTargetViewAsh, DeviceAsh, FenceAsh, SemaphoreAsh};

pub struct SwapChainAsh {
    device: ash::Device,
    swap_chain: ash::extensions::khr::Swapchain,
    swap_chain_khr: ash::vk::SwapchainKHR,
    image_views: Vec<ash::vk::ImageView>,
    current_view_index: u32,
    format: ash::vk::Format,
}

impl SwapChainAsh {
    pub fn new(device: &DeviceAsh, _info: &SwapChainInfo) -> Self {
        let instance = device.get_instance();
        let surface = device.get_surface();
        let surface_loader = device.get_surface_loader();
        let physical_device = device.get_physical_device();
        let device = device.get_device_ref();

        let surface_capabilities = unsafe {
            surface_loader.get_physical_device_surface_capabilities(physical_device, surface)
        }
        .unwrap();
        let surface_format =
            unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface) }
                .unwrap()[0];
        let surface_resolution = match surface_capabilities.current_extent.width {
            std::u32::MAX => ash::vk::Extent2D {
                width: 640,
                height: 480,
            },
            _ => surface_capabilities.current_extent,
        };
        let present_modes = unsafe {
            surface_loader.get_physical_device_surface_present_modes(physical_device, surface)
        }
        .unwrap();
        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == ash::vk::PresentModeKHR::MAILBOX)
            .unwrap_or(ash::vk::PresentModeKHR::FIFO);

        let desired_image_count = surface_capabilities.min_image_count;

        let swap_chain_create_info = ash::vk::SwapchainCreateInfoKHR::builder()
            .surface(surface)
            .min_image_count(desired_image_count)
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(surface_resolution)
            .image_usage(
                ash::vk::ImageUsageFlags::COLOR_ATTACHMENT
                    | ash::vk::ImageUsageFlags::SAMPLED
                    | ash::vk::ImageUsageFlags::TRANSFER_DST,
            )
            .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .image_array_layers(1); // TODO: 物理デバイスに問い合わせた値を使用する

        let swap_chain = ash::extensions::khr::Swapchain::new(&instance, &device);
        let swap_chain_khr =
            unsafe { swap_chain.create_swapchain(&swap_chain_create_info, None) }.unwrap();

        // スキャンバッファの取得
        let images = unsafe { swap_chain.get_swapchain_images(swap_chain_khr) }.unwrap();
        let image_views = images
            .iter()
            .map(|&image| {
                let create_view_info = ash::vk::ImageViewCreateInfo::builder()
                    .view_type(ash::vk::ImageViewType::TYPE_2D)
                    .format(surface_format.format)
                    .components(ash::vk::ComponentMapping {
                        r: ash::vk::ComponentSwizzle::R,
                        g: ash::vk::ComponentSwizzle::G,
                        b: ash::vk::ComponentSwizzle::B,
                        a: ash::vk::ComponentSwizzle::A,
                    })
                    .subresource_range(
                        ash::vk::ImageSubresourceRange::builder()
                            .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                            .base_mip_level(0)
                            .level_count(1)
                            .base_array_layer(0)
                            .layer_count(1)
                            .build(),
                    )
                    .image(image)
                    .build();

                unsafe { device.create_image_view(&create_view_info, None) }.unwrap()
            })
            .collect::<Vec<ash::vk::ImageView>>();

        Self {
            device: device.clone(),
            swap_chain,
            swap_chain_khr,
            image_views,
            current_view_index: 0,
            format: surface_format.format,
        }
    }

    pub fn acquire_next_scan_buffer_view(
        &mut self,
        semaphore: Option<&mut SemaphoreAsh>,
        fence: Option<&mut FenceAsh>,
    ) -> ColorTargetViewAsh {
        let semaphore = match semaphore {
            Some(value) => value.get_semaphore(),
            None => ash::vk::Semaphore::null(),
        };

        let fence = match fence {
            Some(value) => value.get_fence(),
            None => ash::vk::Fence::null(),
        };

        let (index, _) = unsafe {
            self.swap_chain.acquire_next_image(
                self.swap_chain_khr,
                std::u64::MAX, /*timeout*/
                semaphore,
                fence,
            )
        }
        .unwrap();
        let _image_view = self.image_views[index as usize];
        self.current_view_index = index;

        ColorTargetViewAsh::new_from_swap_chain(self)
    }

    pub fn get_swap_chain(&self) -> &ash::extensions::khr::Swapchain {
        &self.swap_chain
    }

    pub fn get_swap_chain_khr(&self) -> ash::vk::SwapchainKHR {
        self.swap_chain_khr
    }

    pub fn get_current_view_index(&self) -> u32 {
        self.current_view_index
    }

    pub fn get_format(&self) -> ash::vk::Format {
        self.format
    }

    pub fn get_image_view(&self, index: usize) -> ash::vk::ImageView {
        self.image_views[index]
    }
}

impl ISwapChain for SwapChainAsh {
    type ColorTargetViewType = ColorTargetViewAsh;
    type DeviceType = DeviceAsh;
    type SemaphoreType = SemaphoreAsh;
    type FenceType = FenceAsh;

    fn new(device: &mut Self::DeviceType, info: &SwapChainInfo) -> Self {
        Self::new(device, info)
    }

    fn acquire_next_scan_buffer_view(
        &mut self,
        semaphore: Option<&mut Self::SemaphoreType>,
        fence: Option<&mut Self::FenceType>,
    ) -> Self::ColorTargetViewType {
        self.acquire_next_scan_buffer_view(semaphore, fence)
    }
}

impl Drop for SwapChainAsh {
    fn drop(&mut self) {
        for image_view in &self.image_views {
            unsafe { self.device.destroy_image_view(*image_view, None) };
        }

        unsafe {
            self.swap_chain.destroy_swapchain(self.swap_chain_khr, None);
        }
    }
}

#[cfg(test)]
mod tests {
    // use sjgfx_interface::{DeviceInfo, SwapChainInfo};

    // use crate::{DeviceAsh, SwapChainAsh};

    // #[test]
    // fn new() {
    //     let device = DeviceAsh::new_with_surface(&DeviceInfo::new());
    //     let _swap_chain = SwapChainAsh::new(&device, &SwapChainInfo::new());
    // }
}
