use crate::GpuAccess;

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
    type DeviceType;

    fn new(device: &mut Self::DeviceType, info: &BufferInfo) -> Self;

    fn map<T, F: Fn(&T)>(&self, func: F);

    fn map_mut<T, F: Fn(&mut T)>(&self, func: F);

    fn map_as_slice<T, F: Fn(&[T])>(&self, func: F);

    fn map_as_slice_mut<T, F: Fn(&mut [T])>(&self, func: F);

    fn flush_mapped_range(&self, offset: isize, size: usize);

    fn invalidate_mapped_range(&self, offset: isize, size: usize);
}
