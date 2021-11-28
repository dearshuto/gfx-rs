use super::super::memory_pool_api::{IMemoryPoolImpl, MemoryPoolInfo};
use super::super::Device;

pub struct MemoryPoolWgpu {}

impl<'a> IMemoryPoolImpl<'a> for MemoryPoolWgpu {
    fn new(_device: &'a Device, _info: &MemoryPoolInfo) -> Self {
        Self {}
    }
}
