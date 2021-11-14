use crate::gfx::Texture;
use crate::gfx::wgpu::color_target_view_wgpu::ColorTargetViewWgpu;

use super::super::swap_chain_api::{ISwapChainImpl, SwapChainInfo};
use super::super::{ColorTargetView, Device};

pub struct SwapChainWgpu<'a> {
	_device: &'a Device,
	_textures: Vec<Texture<'a>>,
	_color_target_views: Vec<ColorTargetView<'a>>,
}

impl<'a> SwapChainWgpu<'a> {
	pub fn get_scan_buffer_texture(&self) -> &wgpu::Texture {
		todo!()
		//&self._texture
	}
}

impl<'a> ISwapChainImpl<'a> for SwapChainWgpu<'a> {
    fn new(device: &'a Device, info: &'a mut SwapChainInfo<'a>) -> Self {
		let adapter = device.to_data().get_adapter();
		let surface = device.to_data().get_surface().as_ref().unwrap();
		let swapchain_format = surface.get_preferred_format(adapter).unwrap();
		let _layer = info.get_layer();
		
		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: swapchain_format,
			width: 640,
			height: 480,
			present_mode: wgpu::PresentMode::Mailbox,
		};
		surface.configure(&device.to_data().get_device(), &config);

		let current_scan_buffer = device.to_data().get_surface().as_ref().unwrap().get_current_frame().unwrap().output;
		let current_scan_buffer_view = current_scan_buffer.texture.create_view(&wgpu::TextureViewDescriptor::default());
		let mut color_target_view = ColorTargetView::new_internal(ColorTargetViewWgpu::new_from_swap_chain(device));
		color_target_view.to_data_mut().set_texture_view(current_scan_buffer_view);

		Self {
			_device: device,
			_textures: Vec::new(),
			_color_target_views: vec![color_target_view],
		}
    }

    fn get_scan_buffer_views_mut(&mut self) -> &mut [ColorTargetView<'a>] {
        &mut self._color_target_views
    }

    fn get_scan_buffers_mut(&mut self) -> &mut [crate::gfx::Texture<'a>] {
        &mut self._textures
    }

    fn get_scan_buffers_and_views(
        &mut self,
    ) -> (&mut [crate::gfx::Texture<'a>], &mut [ColorTargetView<'a>]) {
        (&mut self._textures, &mut self._color_target_views)
    }

    fn acquire_next_scan_buffer_index(
        &mut self,
        _semaphore: Option<&mut crate::gfx::Semaphore>,
        _fence: Option<&mut crate::gfx::Fence>,
    ) -> i32 {
		0
    }

    fn update(&mut self) {
		let current_scan_buffer = self._device.to_data().get_surface().as_ref().unwrap().get_current_frame().unwrap().output;
		let current_scan_buffer_view = current_scan_buffer.texture.create_view(&wgpu::TextureViewDescriptor::default());

		self._color_target_views[0].to_data_mut().set_texture_view(current_scan_buffer_view);
    }
}
