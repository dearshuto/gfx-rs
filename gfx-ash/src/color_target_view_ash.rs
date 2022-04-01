use sjgfx_interface::IColorTargetView;

use crate::{DeviceAsh, SwapChainAsh};

pub struct ColorTargetViewAsh {
    format: Option<ash::vk::Format>,
    image: Option<ash::vk::Image>,
    image_view: Option<ash::vk::ImageView>,
}

impl ColorTargetViewAsh {
    pub(crate) fn new_from_swap_chain(swap_chain: &SwapChainAsh) -> Self {
        let index = swap_chain.get_current_view_index() as usize;
        let image_view = swap_chain.get_image_view(index);
        let image = swap_chain.get_image(index);

        Self {
            format: Some(swap_chain.get_format()),
            image: Some(image),
            image_view: Some(image_view),
        }
    }

    pub fn get_format(&self) -> ash::vk::Format {
        self.format.unwrap()
    }

    pub fn get_image(&self) -> ash::vk::Image {
        self.image.unwrap()
    }

    pub fn get_image_view(&self) -> ash::vk::ImageView {
        self.image_view.unwrap()
    }
}

impl IColorTargetView for ColorTargetViewAsh {
    type DeviceType = DeviceAsh;

    fn new(_device: &Self::DeviceType, _info: &sjgfx_interface::ColorTargetViewInfo) -> Self {
        todo!()
    }
}
