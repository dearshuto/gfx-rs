use super::super::super::vi::Layer;
use super::super::swap_chain_api::{ISwapChainImpl, SwapChainInfo};
use super::super::{ColorTargetView, Device};
use winit::platform::run_return::EventLoopExtRunReturn;

pub struct SwapChainImpl<'a> {
    _swap_chain: ash::vk::SwapchainKHR,
    _layer: &'a mut Layer,
    _scan_buffer_views: Vec<ColorTargetView<'a>>,
}

impl<'a> ISwapChainImpl<'a> for SwapChainImpl<'a> {
    fn new(device: &Device, info: &'a mut SwapChainInfo<'a>) -> Self {
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
            let surface_loader = ash::extensions::khr::Surface::new(entry, instance);
            let surface_capabilities = surface_loader
                .get_physical_device_surface_capabilities(*physical_device, surface)
                .unwrap();
            let surface_format = surface_loader
                .get_physical_device_surface_formats(*physical_device, surface)
                .unwrap()[0];
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
                .min_image_count(1)
                .image_color_space(surface_format.color_space)
                .image_format(surface_format.format)
                .image_extent(ash::vk::Extent2D::builder().width(800).height(600).build())
                .image_usage(ash::vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
                .pre_transform(surface_capabilities.current_transform)
                .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
                .present_mode(present_mode)
                .clipped(true)
                .image_array_layers(1)
                .min_image_count(2); // TODO: 物理デバイスに問い合わせた値を使用する

            let swap_chain = swap_chain_loader
                .create_swapchain(&swap_chain_create_info, None)
                .unwrap();
            Self {
                _swap_chain: swap_chain,
                _layer: layer,
                _scan_buffer_views: Vec::new(),
            }
        }
    }

    fn get_scan_buffer_views_mut(&mut self) -> &'a mut [ColorTargetView] {
        &mut self._scan_buffer_views
    }

    fn update(&mut self) {
        let event_loop = &mut self._layer.get_event_loop_mut();

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
