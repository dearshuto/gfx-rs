use crate::gfx::semaphore_api::ISemaphore;

pub struct SemaphoreVk {}

impl<'a> ISemaphore<'a> for SemaphoreVk {
    fn new(_device: &'a crate::gfx::Device, _info: &crate::gfx::SemaphoreInfo) -> Self {
        Self {}
    }
}
