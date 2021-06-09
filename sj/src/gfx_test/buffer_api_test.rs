#[cfg(test)]
mod tests {
    use super::super::super::gfx::{
        Buffer, BufferInfo, BufferUsage, Device, DeviceInfo, MemoryPool, MemoryPoolInfo,
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
        let _buffer = Buffer::new(
            &device,
            &BufferInfo::new()
                .set_size(512)
                .set_buffer_usage(BufferUsage::CONSTANT_BUFFER),
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
        let _buffer = Buffer::new(
            &device,
            &BufferInfo::new()
                .set_size(512)
                .set_buffer_usage(BufferUsage::CONSTANT_BUFFER),
            &memory_pool,
            0,
            512,
        );
    }
}
