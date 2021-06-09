#[cfg(test)]
mod tests {
    use crate::gfx::DeviceInfo;

    use super::super::super::gfx::{Device, MemoryPool, MemoryPoolInfo, MemoryPoolProperty};

    #[test]
    fn initialize() {
        let device = Device::new(&DeviceInfo::new());
        let _memory_pool = MemoryPool::new(
            &device,
            &MemoryPoolInfo::new()
                .set_size(1024)
                .set_memory_pool_property(
                    MemoryPoolProperty::CPU_CACHED | MemoryPoolProperty::GPU_CACHED,
                ),
        );
        let _memory_pool = MemoryPool::new(
            &device,
            &MemoryPoolInfo::new()
                .set_size(1024)
                .set_memory_pool_property(
                    MemoryPoolProperty::CPU_UNCACHED | MemoryPoolProperty::GPU_CACHED,
                ),
        );
        let _memory_pool = MemoryPool::new(
            &device,
            &MemoryPoolInfo::new()
                .set_size(1024)
                .set_memory_pool_property(
                    MemoryPoolProperty::CPU_INVISIBLE | MemoryPoolProperty::GPU_CACHED,
                ),
        );
    }
}
