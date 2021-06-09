use crate::gfx::semaphore_api::ISemaphore;

pub struct SemaphoreWgpu {}

impl<'a> ISemaphore<'a> for SemaphoreWgpu {
    fn new(_device: &'a crate::gfx::Device, _info: &crate::gfx::SemaphoreInfo) -> Self {
        Self {}
    }
}
