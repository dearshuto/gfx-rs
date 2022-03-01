use sjgfx_interface::FenceInfo;
use vulkano::sync::{self, GpuFuture};

use crate::DeviceVk;

pub struct FenceVk {
    previous_frame_end: Option<Box<dyn GpuFuture>>,
}

impl FenceVk {
    pub fn new(device: &DeviceVk, _info: &FenceInfo) -> Self {
        Self {
            previous_frame_end: Some(sync::now(device.clone_device()).boxed()),
        }
    }

    pub(crate) fn cleanup_finished(&mut self) {
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();
    }
}
