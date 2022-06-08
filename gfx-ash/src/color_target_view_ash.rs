use sjgfx_interface::{ColorTargetViewInfo, IColorTargetView, TextureViewInfo};

use crate::{util, DeviceAsh, SwapChainAsh, TextureAsh, TextureViewAsh};

pub struct ColorTargetViewAsh {
    format: Option<ash::vk::Format>,
    image: Option<ash::vk::Image>,
    image_view: Option<ash::vk::ImageView>,
    width: u32,
    height: u32,

    // テクスチャから初期化した場合に使う
    texture_view: Option<TextureViewAsh>,
}

impl ColorTargetViewAsh {
    pub fn new(device: &DeviceAsh, info: &ColorTargetViewInfo, texture: &TextureAsh) -> Self {
        let texture_view = TextureViewAsh::new(
            device,
            &TextureViewInfo::new().set_format(info.get_image_format()),
            texture,
        );
        let format = util::convert_image_format(info.get_image_format());

        Self {
            format: Some(format),
            image: None,
            image_view: None,
            width: 640,
            height: 480,
            texture_view: Some(texture_view),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_swap_chain(swap_chain: &SwapChainAsh) -> Self {
        let index = swap_chain.get_current_view_index() as usize;
        let image_view = swap_chain.get_image_view(index);
        let image = swap_chain.get_image(index);

        Self {
            format: Some(swap_chain.get_format()),
            image: Some(image),
            image_view: Some(image_view),
            width: swap_chain.get_width(),
            height: swap_chain.get_height(),

            texture_view: None,
        }
    }

    pub fn get_format(&self) -> ash::vk::Format {
        self.format.unwrap()
    }

    pub fn get_image(&self) -> ash::vk::Image {
        self.image.unwrap()
    }

    pub fn get_image_view(&self) -> ash::vk::ImageView {
        if let Some(scan_buffer) = self.image_view {
            scan_buffer
        } else if let Some(image_view) = &self.texture_view {
            image_view.get_image_view()
        } else {
            panic!()
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

impl IColorTargetView for ColorTargetViewAsh {
    type DeviceType = DeviceAsh;
    type TextureType = TextureAsh;

    fn new(
        device: &Self::DeviceType,
        info: &sjgfx_interface::ColorTargetViewInfo,
        texture: &Self::TextureType,
    ) -> Self {
        Self::new(device, info, texture)
    }
}
