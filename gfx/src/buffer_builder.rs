use sjgfx_interface::{BufferInfo, GpuAccess, IBuffer};

use crate::api::IApi;

pub struct TBufferBuilder<T: IApi> {
    info: BufferInfo,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IApi> TBufferBuilder<T> {
    pub fn new() -> Self {
        Self {
            info: BufferInfo::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &T::Device) -> T::Buffer {
        T::Buffer::new(device, &self.info)
    }

    pub fn with_size(self, size: usize) -> Self {
        Self {
            info: self.info.set_size(size),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn enable_vertex_buffer(self) -> Self {
        self.enable_flag(GpuAccess::VERTEX_BUFFER)
    }

    pub fn enable_index_buffer(self) -> Self {
        self.enable_flag(GpuAccess::INDEX_BUFFER)
    }

    pub fn enable_constant_buffer(self) -> Self {
        self.enable_flag(GpuAccess::CONSTANT_BUFFER)
    }

    fn enable_flag(self, flag: GpuAccess) -> Self {
        let gpu_access = self.info.get_gpu_access_flags() | flag;
        Self {
            info: self.info.set_gpu_access_flags(gpu_access),
            _marker: std::marker::PhantomData,
        }
    }
}
