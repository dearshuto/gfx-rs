use crate::gfx::color_target_view_api::{ColorTargetViewInfo, IColorTargetViewImpl};
use crate::gfx::Device;
use std::sync::Arc;
use vulkano::image::ImageAccess;

pub struct ColorTargetViewVk<'a> {
    _device: &'a Device,
    _image_access: Arc<dyn ImageAccess>,
}

impl<'a> IColorTargetViewImpl<'a> for ColorTargetViewVk<'a> {
    fn new(device: &'a Device, info: &'a ColorTargetViewInfo) -> Self {
        let image_access = info.get_texture().to_data().clone_image_acess();

        Self {
            _device: device,
            _image_access: image_access,
        }
    }
}

impl<'a> ColorTargetViewVk<'a> {
    pub fn clone_image(&self) -> std::sync::Arc<dyn ImageAccess> {
        self._image_access.clone()
    }
}
