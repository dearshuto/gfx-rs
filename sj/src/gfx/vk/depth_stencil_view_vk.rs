use crate::gfx::depth_stencil_view_api::{DepthStencilViewInfo, IDepthStencilView};

pub struct DepthStencilViewVk {
    _image_view: std::sync::Arc<vulkano::image::ImageViewAbstract>,
}

impl<'a> IDepthStencilView<'a> for DepthStencilViewVk {
    fn new(device: &'a crate::gfx::Device, _info: &DepthStencilViewInfo) -> Self {
        let device_vk = device.to_data().get_device_impl();
        let dimensions = [128, 128];
        let image_view = vulkano::image::view::ImageView::new(
            vulkano::image::AttachmentImage::transient(
                device_vk.clone(),
                dimensions,
                vulkano::format::Format::D32Sfloat,
            )
            .unwrap(),
        )
        .unwrap();

        Self {
            _image_view: std::sync::Arc::new(image_view),
        }
    }
}
