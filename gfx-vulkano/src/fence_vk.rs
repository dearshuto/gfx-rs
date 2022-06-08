use sjgfx_interface::{FenceInfo, IFence};
use vulkano::sync::{self, GpuFuture};

use crate::DeviceVk;

pub struct FenceVk {
    #[allow(dead_code)]
    previous_frame_end: Option<Box<dyn GpuFuture>>,
}

impl FenceVk {
    pub fn new(device: &DeviceVk, _info: &FenceInfo) -> Self {
        Self {
            previous_frame_end: Some(sync::now(device.clone_device()).boxed()),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn cleanup_finished(&mut self) {
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();
    }
}

impl IFence for FenceVk {
    type DeviceType = DeviceVk;

    fn new(device: &Self::DeviceType, info: &FenceInfo) -> Self {
        Self::new(device, info)
    }
}
