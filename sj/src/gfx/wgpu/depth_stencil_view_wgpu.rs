use super::super::depth_stencil_view_api::{DepthStencilViewInfo, IDepthStencilView};
use super::super::Device;

pub struct DepthStencilViewWgpu<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> IDepthStencilView<'a> for DepthStencilViewWgpu<'a> {
    fn new(device: &Device, info: &DepthStencilViewInfo) -> Self {
        todo!();
    }
}
