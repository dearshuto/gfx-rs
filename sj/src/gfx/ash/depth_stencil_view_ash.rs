use super::super::depth_stencil_view_api::{DepthStencilViewInfo, IDepthStencilView};
use super::super::Device;

pub struct DepthStencilViewImpl {}

impl IDepthStencilView for DepthStencilViewImpl {
    fn new(_device: &Device, _info: &DepthStencilViewInfo) -> Self {
        std::unimplemented!();
    }
}
