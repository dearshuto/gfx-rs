use super::super::depth_stencil_view_api::{DepthStencilViewInfo, IDepthStencilView};
use super::super::Device;

pub struct DepthStencilViewWgpu {}

impl<'a> IDepthStencilView<'a> for DepthStencilViewWgpu {
    fn new(_device: &Device, _info: &DepthStencilViewInfo) -> Self {
        todo!()
    }
}
