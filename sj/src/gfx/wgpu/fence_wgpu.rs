use crate::gfx::fence_api::IFence;

pub struct FenceWgpu {}

impl<'a> IFence<'a> for FenceWgpu {
    fn new(_device: &'a crate::gfx::Device, _info: &crate::gfx::FenceInfo) -> Self {
        Self {}
    }
}
