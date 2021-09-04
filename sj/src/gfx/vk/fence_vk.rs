use crate::gfx::fence_api::{FenceInfo, IFence};

pub struct Fence {}

impl<'a> IFence<'a> for Fence {
    fn new(_device: &'a crate::gfx::Device, _info: &FenceInfo) -> Self {
        Self {}
    }
}
