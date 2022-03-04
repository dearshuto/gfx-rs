use std::sync::Arc;

use sjgfx_interface::SwapChainInfo;
use vulkano::{
    image::{view::ImageView, ImageUsage, ImageViewAbstract, SwapchainImage},
    swapchain::{self, AcquireError, Swapchain, SwapchainAcquireFuture},
};
use winit::window::Window;

use crate::{ColorTargetViewVk, DeviceVk, FenceVk};

pub struct SwapChainVk {
    swap_chain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    swap_chain_acquire_future: Option<SwapchainAcquireFuture<Window>>,
    index: i32,
}

impl SwapChainVk {
    pub fn new(device: &DeviceVk, _info: &SwapChainInfo) -> Self {
        let surface = device.clone_surface();
        let physical_device = device.get_physical_device();

        let capabilities = surface.capabilities(physical_device).unwrap();
        let composite_alpha = capabilities
            .supported_composite_alpha
            .iter()
            .next()
            .unwrap();
        let format = capabilities.supported_formats[0].0;
        let dimensions: [u32; 2] = surface.window().inner_size().into();

        let (swap_chain, images) = Swapchain::start(device.clone_device(), surface.clone())
            .num_images(capabilities.min_image_count)
            .format(format)
            .dimensions(dimensions)
            .usage(ImageUsage::color_attachment())
            .sharing_mode(&device.clone_queue())
            .composite_alpha(composite_alpha)
            .build()
            .unwrap();

        Self {
            swap_chain,
            images,
            swap_chain_acquire_future: None,
            index: -1,
        }
    }

    pub fn get_color_target_views(&self) -> Vec<ColorTargetViewVk> {
        todo!()
    }

    pub fn acquire_next_scan_buffer_index(&mut self) -> i32 {
        let (image_num, _suboptimal, acquire_future) =
            match swapchain::acquire_next_image(self.swap_chain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    return -1;
                }
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };

        self.swap_chain_acquire_future = Some(acquire_future);
        self.index = image_num as i32;
        image_num as i32
    }

    pub fn acquire_next_scan_buffer_view<'b>(
        &mut self,
        fence: &mut FenceVk,
    ) -> ColorTargetViewVk<'b> {
        fence.cleanup_finished();

        let (image_num, _suboptimal, acquire_future) =
            match swapchain::acquire_next_image(self.swap_chain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    todo!()
                }
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };

        self.swap_chain_acquire_future = Some(acquire_future);
        self.index = image_num as i32;

        ColorTargetViewVk::new_from_swap_chain(self)
    }

    pub(crate) fn clone_current_image_view(&self) -> Arc<dyn ImageViewAbstract> {
        let index = self.index as usize;
        ImageView::new(self.images[index].clone()).unwrap()
    }

    pub fn unwrap_feature(&mut self) -> SwapchainAcquireFuture<Window> {
        let mut temp = None;
        std::mem::swap(&mut temp, &mut self.swap_chain_acquire_future);
        temp.unwrap()
    }

    pub fn get_current_index(&self) -> i32 {
        self.index
    }

    pub fn get_swap_chain(&self) -> &Swapchain<Window> {
        self.swap_chain.as_ref()
    }

    pub fn clone_swap_chain(&self) -> Arc<Swapchain<Window>> {
        self.swap_chain.clone()
    }

    pub fn unwrap_acquire_future(&mut self) -> SwapchainAcquireFuture<Window> {
        let mut temp = None;
        std::mem::swap(&mut temp, &mut self.swap_chain_acquire_future);
        temp.unwrap()
    }
}
