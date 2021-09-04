use crate::gfx::memory_pool_api::IMemoryPoolImpl;
use crate::gfx::Device;

pub struct MemoryPoolVk {}

impl<'a> IMemoryPoolImpl<'a> for MemoryPoolVk {
    fn new(_device: &'a Device, _info: &crate::gfx::MemoryPoolInfo) -> Self {
        Self {}
    }
}
