use crate::SwapChainAsh;

pub struct ColorTargetViewAsh {
    format: Option<ash::vk::Format>,
    image_view: Option<ash::vk::ImageView>,
}

impl ColorTargetViewAsh {
    pub(crate) fn new_from_swap_chain(swap_chain: &SwapChainAsh) -> Self {
        let index = swap_chain.get_current_view_index() as usize;
        let image_view = swap_chain.get_image_view(index);

        Self {
            format: Some(swap_chain.get_format()),
            image_view: Some(image_view),
        }
    }

    pub fn get_format(&self) -> ash::vk::Format {
        self.format.unwrap()
    }

    pub fn get_image_view(&self) -> ash::vk::ImageView {
        self.image_view.unwrap()
    }
}
