use std::sync::Arc;

use sjgfx_interface::{IDisplayEventListener, ISwapChain, SwapChainInfo};
use vulkano::{
    image::{view::ImageView, ImageUsage, ImageViewAbstract, SwapchainImage},
    swapchain::{
        self, AcquireError, CompositeAlpha, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo,
    },
};

use crate::{ColorTargetViewVk, DeviceVk, FenceVk, SemaphoreVk};

pub struct SwapChainVk {
    swap_chain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    swap_chain_acquire_future: Option<SwapchainAcquireFuture>,
    index: i32,
}

impl SwapChainVk {
    pub fn new(device: &DeviceVk, _info: &SwapChainInfo) -> Self {
        let surface = device.clone_surface();
        let physical_device = device.get_physical_device();

        let capabilities = physical_device
            .surface_capabilities(&surface, Default::default())
            .unwrap();

        // MEMO: Swapchain がサポートしているかのチェックをした方がいいと思う
        let _composite_alpha = capabilities.supported_composite_alpha;

        let image_format = physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        let (swap_chain, images) = Swapchain::new(
            device.clone_device(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: capabilities.min_image_count,
                image_format: Some(image_format),
                // image_extent: surface.window().inner_size().into(),
                image_extent: [1280, 960],
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha: CompositeAlpha::Opaque,
                ..Default::default()
            },
        )
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

    pub fn acquire_next_scan_buffer_index(
        &mut self,
        _semaphore: Option<&mut SemaphoreVk>,
        _fence: Option<&mut FenceVk>,
    ) -> i32 {
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

    pub fn acquire_next_scan_buffer_view(
        &mut self,
        _semaphore: Option<&mut SemaphoreVk>,
        fence: Option<&mut FenceVk>,
    ) -> ColorTargetViewVk {
        if let Some(fence) = fence {
            fence.cleanup_finished();
        }

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
        ImageView::new_default(self.images[index].clone()).unwrap()
    }

    pub fn unwrap_feature(&mut self) -> SwapchainAcquireFuture {
        let mut temp = None;
        std::mem::swap(&mut temp, &mut self.swap_chain_acquire_future);
        temp.unwrap()
    }

    pub fn get_current_index(&self) -> i32 {
        self.index
    }

    pub fn get_swap_chain(&self) -> &Swapchain {
        self.swap_chain.as_ref()
    }

    pub fn clone_swap_chain(&self) -> Arc<Swapchain> {
        self.swap_chain.clone()
    }

    pub fn unwrap_acquire_future(&mut self) -> SwapchainAcquireFuture {
        let mut temp = None;
        std::mem::swap(&mut temp, &mut self.swap_chain_acquire_future);
        temp.unwrap()
    }
}

impl ISwapChain for SwapChainVk {
    type ColorTargetViewType = ColorTargetViewVk;
    type DeviceType = DeviceVk;
    type SemaphoreType = SemaphoreVk;
    type FenceType = FenceVk;

    fn new(device: &mut Self::DeviceType, info: &SwapChainInfo) -> Self {
        Self::new(device, info)
    }

    fn acquire_next_scan_buffer_view(
        &mut self,
        _semaphore: Option<&mut Self::SemaphoreType>,
        _fence: Option<&mut Self::FenceType>,
    ) -> &mut Self::ColorTargetViewType {
        todo!()
        // self.acquire_next_scan_buffer_view(semaphore, fence)
    }
}

impl IDisplayEventListener for SwapChainVk {
    fn on_resized(&mut self, _width: u32, _height: u32) {}
}
