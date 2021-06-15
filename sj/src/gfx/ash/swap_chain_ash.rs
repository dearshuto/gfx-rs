use ash::version::{DeviceV1_0, InstanceV1_0};

use crate::gfx::{Fence, Semaphore};

use super::super::super::vi::Layer;
use super::super::swap_chain_api::{ISwapChainImpl, SwapChainInfo};
use super::super::{ColorTargetView, Device, Texture};
use super::color_target_view_ash::ColorTargetViewImpl;
use super::texture_ash::TextureImpl;
use winit::platform::run_return::EventLoopExtRunReturn;

pub struct SwapChainImpl<'a> {
    _device: &'a Device,
    _surface: ash::vk::SurfaceKHR,
    _surface_loader: ash::extensions::khr::Surface,
    _swap_chain: ash::vk::SwapchainKHR,
    _swap_chain_loader: ash::extensions::khr::Swapchain,
    _layer: &'a mut Layer,
    _image_views: Vec<ash::vk::ImageView>,
    _textures: Vec<Texture<'a>>,
    _scan_buffer_views: Vec<ColorTargetView<'a>>,
    _buffer_index: i32,
}

impl<'a> SwapChainImpl<'a> {
    pub fn get_swap_chain_khr(&self) -> ash::vk::SwapchainKHR {
        self._swap_chain
    }

    pub fn get_swap_chain(&self) -> &ash::extensions::khr::Swapchain {
        &self._swap_chain_loader
    }

    pub fn get_buffer_index(&self) -> i32 {
        self._buffer_index
    }
}

impl<'a> ISwapChainImpl<'a> for SwapChainImpl<'a> {
    fn new(device: &'a Device, info: &'a mut SwapChainInfo<'a>) -> Self {
        let entry: &ash::EntryCustom<std::sync::Arc<libloading::Library>> =
            device.to_data().get_entry();
        let instance = device.to_data().get_instance();
        let layer = info.get_layer();
        let physical_device = device.to_data().get_physical_device();
        let device_impl = device.to_data().get_device();
        let window = layer.get_window();
        let swap_chain_loader = ash::extensions::khr::Swapchain::new(instance, device_impl);

        unsafe {
            let surface = ash_window::create_surface(entry, instance, window, None).unwrap();
            let _pdevices = instance
                .enumerate_physical_devices()
                .expect("Physical device error");
            let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

            // 呼ばないとだめらしい
            let _ = surface_loader
                .get_physical_device_surface_support(
                    *device.to_data().get_physical_device(),
                    0,
                    surface,
                )
                .unwrap();

            let surface_capabilities = surface_loader
                .get_physical_device_surface_capabilities(*physical_device, surface)
                .unwrap();
            let surface_format = surface_loader
                .get_physical_device_surface_formats(*physical_device, surface)
                .unwrap()[0];
            let surface_resolution = match surface_capabilities.current_extent.width {
                std::u32::MAX => ash::vk::Extent2D {
                    width: 640,
                    height: 480,
                },
                _ => surface_capabilities.current_extent,
            };
            let present_modes = surface_loader
                .get_physical_device_surface_present_modes(*physical_device, surface)
                .unwrap();
            let present_mode = present_modes
                .iter()
                .cloned()
                .find(|&mode| mode == ash::vk::PresentModeKHR::MAILBOX)
                .unwrap_or(ash::vk::PresentModeKHR::FIFO);

            let swap_chain_create_info = ash::vk::SwapchainCreateInfoKHR::builder()
                .surface(surface)
                .min_image_count(2)
                .image_color_space(surface_format.color_space)
                .image_format(surface_format.format)
                .image_extent(surface_resolution)
                .image_usage(
                    ash::vk::ImageUsageFlags::COLOR_ATTACHMENT
                        | ash::vk::ImageUsageFlags::INPUT_ATTACHMENT
                        | ash::vk::ImageUsageFlags::TRANSFER_SRC
                        | ash::vk::ImageUsageFlags::TRANSFER_DST,
                )
                .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
                .pre_transform(surface_capabilities.current_transform)
                .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
                .present_mode(present_mode)
                .clipped(true)
                .image_array_layers(1); // TODO: 物理デバイスに問い合わせた値を使用する

            let swap_chain = swap_chain_loader
                .create_swapchain(&swap_chain_create_info, None)
                .unwrap();
            let present_images = swap_chain_loader.get_swapchain_images(swap_chain).unwrap();
            let present_image_views = present_images
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

                    device_impl
                        .create_image_view(&create_view_info, None)
                        .unwrap()
                })
                .collect::<Vec<ash::vk::ImageView>>();

            let textures = present_images
                .iter()
                .map(|image| {
                    let texture_impl = TextureImpl::<'a>::new(device, *image, 640, 480);
                    let texture = Texture::new_internal(texture_impl);
                    texture
                })
                .collect::<Vec<Texture<'a>>>();

            // TODO: イテレータを使って書きたい
            let mut color_target_views = Vec::<ColorTargetView<'a>>::new();
            for i in 0..textures.len() {
                let color_target_view_impl = ColorTargetViewImpl::new(
                    device,
                    present_images[i],
                    640,
                    480,
                    present_image_views[i],
                    ash::vk::Format::B8G8R8A8_UNORM,
                );
                let color_target_view = ColorTargetView::new_internal(color_target_view_impl);
                color_target_views.push(color_target_view);
            }

            Self {
                _device: device,
                _swap_chain: swap_chain,
                _surface: surface,
                _surface_loader: surface_loader,
                _swap_chain_loader: swap_chain_loader,
                _layer: layer,
                _image_views: present_image_views,
                _scan_buffer_views: color_target_views,
                _textures: textures,
                _buffer_index: -1,
            }
        }
    }

    fn get_scan_buffer_views_mut(&mut self) -> &mut [ColorTargetView<'a>] {
        &mut self._scan_buffer_views
    }

    fn get_scan_buffers_mut(&mut self) -> &mut [Texture<'a>] {
        &mut self._textures
    }

    fn get_scan_buffers_and_views(&mut self) -> (&mut [Texture<'a>], &mut [ColorTargetView<'a>]) {
        (&mut self._textures, &mut self._scan_buffer_views)
    }

    fn acquire_next_scan_buffer_index(
        &mut self,
        semaphore: Option<&mut Semaphore>,
        fence: Option<&mut Fence>,
    ) -> i32 {
        let semaphore_ash = match semaphore {
            Some(value) => value.to_data_mut().get_semaphore(),
            None => ash::vk::Semaphore::null(),
        };

        let fence_ash = match fence {
            Some(value) => value.to_data().get_fence(),
            None => ash::vk::Fence::null(),
        };

        unsafe {
            let (index, _) = self
                ._swap_chain_loader
                .acquire_next_image(self._swap_chain, std::u64::MAX, semaphore_ash, fence_ash)
                .unwrap();

            self._buffer_index = index as i32;
            index as i32
        }
    }

    fn update(&mut self) {
        let (event_loop, _window) = &mut self._layer.get_event_loop_and_window_mut();

        event_loop.run_return(|event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Wait;
            if let winit::event::Event::WindowEvent { event: _, .. } = &event {
                // Print only Window events to reduce noise
                //println!("{:?}", event);
            }

            match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    //quit = true;
                }
                winit::event::Event::MainEventsCleared => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                _ => (),
            }
        });
    }
}

impl<'a> Drop for SwapChainImpl<'a> {
    fn drop(&mut self) {
        let swap_chain_loader = &self._swap_chain_loader;
        unsafe {
            swap_chain_loader.destroy_swapchain(self._swap_chain, None);
            self._surface_loader.destroy_surface(self._surface, None);
        }
    }
}
