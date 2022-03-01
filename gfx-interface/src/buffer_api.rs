use crate::{IDevice, GpuAccess};

pub struct BufferInfo {
    size: usize,
    gpu_access_flags: GpuAccess,
}

impl BufferInfo {
    pub fn new() -> Self {
        BufferInfo {
            size: 0,
            gpu_access_flags: GpuAccess::empty(),
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn set_size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    pub fn get_gpu_access_flags(&self) -> GpuAccess {
        self.gpu_access_flags
    }

    pub fn set_gpu_access_flags(mut self, buffer_usage: GpuAccess) -> Self {
        self.gpu_access_flags = buffer_usage;
        self
    }
}

pub trait IBuffer {
    fn new<TDevice: IDevice>(device: &TDevice) -> Self;
}
