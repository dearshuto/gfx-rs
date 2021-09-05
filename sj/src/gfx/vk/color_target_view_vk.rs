use crate::gfx::color_target_view_api::{ColorTargetViewInfo, IColorTargetViewImpl};
use crate::gfx::Device;
use std::sync::Arc;
use vulkano::image::view::{ImageView, ImageViewAbstract};

pub struct ColorTargetViewVk<'a> {
    _device: &'a Device,
    _image_view: Arc<dyn ImageViewAbstract>,
}

impl<'a> IColorTargetViewImpl<'a> for ColorTargetViewVk<'a> {
    fn new(device: &'a Device, info: &'a ColorTargetViewInfo) -> Self {
        let image_access = info.get_texture().to_data().clone_image_acess();
        let image_view = ImageView::new(image_access).unwrap();

        Self {
            _device: device,
            _image_view: image_view,
        }
    }
}
