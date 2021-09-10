#[cfg(test)]
mod tests {
    use super::super::super::gfx::{
        Buffer, BufferInfo, Device, DeviceInfo, GpuAccess, MemoryPool, MemoryPoolInfo,
        MemoryPoolProperty,
    };

    #[test]
    fn initialize() {
        let device = Device::new(&DeviceInfo::new());
        let memory_pool = MemoryPool::new(
            &device,
            &MemoryPoolInfo::new()
                .set_size(1024)
                .set_memory_pool_property(
                    MemoryPoolProperty::CPU_CACHED | MemoryPoolProperty::GPU_CACHED,
                ),
        );
        let _buffer = Buffer::<i32>::new(
            &device,
            &BufferInfo::new()
                .set_size(512)
                .set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER),
            &memory_pool,
            0,
            512,
        );
    }

    #[test]
    #[should_panic]
    fn less_memory_pool() {
        let device = Device::new(&DeviceInfo::new());
        let memory_pool = MemoryPool::new(
            &device,
            &MemoryPoolInfo::new().set_size(16).set_memory_pool_property(
                MemoryPoolProperty::CPU_CACHED | MemoryPoolProperty::GPU_CACHED,
            ),
        );
        let _buffer = Buffer::<i32>::new(
            &device,
            &BufferInfo::new()
                .set_size(512)
                .set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER),
            &memory_pool,
            0,
            16,
        );
    }
}
