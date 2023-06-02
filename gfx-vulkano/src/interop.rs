use sjgfx_interface::GpuAccess;
use vulkano::buffer::BufferUsage;

pub fn convert_usage(gpu_access: &GpuAccess) -> BufferUsage {
    let mut buffer_usage = Default::default();
    if gpu_access.contains(GpuAccess::CONSTANT_BUFFER) {
        buffer_usage |= BufferUsage::UNIFORM_BUFFER;
    }
    if gpu_access.contains(GpuAccess::UNORDERED_ACCESS_BUFFER) {
        buffer_usage |= BufferUsage::STORAGE_BUFFER;
    }
    if gpu_access.contains(GpuAccess::INDEX_BUFFER) {
        buffer_usage |= BufferUsage::INDEX_BUFFER;
    }
    if gpu_access.contains(GpuAccess::VERTEX_BUFFER) {
        buffer_usage |= BufferUsage::VERTEX_BUFFER;
    }
    if gpu_access.contains(GpuAccess::INDIRECT_BUFFER) {
        buffer_usage |= BufferUsage::INDIRECT_BUFFER;
    }

    buffer_usage
}
