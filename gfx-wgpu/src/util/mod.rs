mod convert_pipeline_layout;

pub use convert_pipeline_layout::{
    create_bind_group_layout, create_bind_group_layout_entries, create_pipeline_layout,
    create_vertex_attributes,
};
use sjgfx_interface::{AttributeFormat, GpuAccess, ImageFormat, ShaderStage};

pub fn convert_to_buffer_usage(gpu_access: GpuAccess) -> wgpu::BufferUsages {
    let mut result = wgpu::BufferUsages::empty();
    if gpu_access.contains(GpuAccess::VERTEX_BUFFER) {
        result |= wgpu::BufferUsages::VERTEX;
        result |= wgpu::BufferUsages::COPY_DST;
    }
    if gpu_access.contains(GpuAccess::INDEX_BUFFER) {
        result |= wgpu::BufferUsages::INDEX;
        result |= wgpu::BufferUsages::COPY_DST;
    }
    if gpu_access.contains(GpuAccess::UNORDERED_ACCESS_BUFFER) {
        result = wgpu::BufferUsages::STORAGE;
    }
    if gpu_access.contains(GpuAccess::CONSTANT_BUFFER) {
        result |= wgpu::BufferUsages::UNIFORM;
        result |= wgpu::BufferUsages::COPY_DST;
    }

    if gpu_access.contains(GpuAccess::READ) {
        result |= wgpu::BufferUsages::COPY_SRC;
        result |= wgpu::BufferUsages::COPY_DST;
    }

    if gpu_access.contains(GpuAccess::WRITE) {
        result |= wgpu::BufferUsages::MAP_READ;
        result |= wgpu::BufferUsages::COPY_DST;
    }

    result
}

pub fn convert_format(format: ImageFormat) -> wgpu::TextureFormat {
    match format {
        ImageFormat::R8Unorm => wgpu::TextureFormat::R8Unorm,
        ImageFormat::R8Snorm => wgpu::TextureFormat::R8Snorm,
        ImageFormat::R8Uint => wgpu::TextureFormat::R8Uint,
        ImageFormat::R8Sint => wgpu::TextureFormat::R8Sint,
        ImageFormat::R32Uint => wgpu::TextureFormat::R32Uint,
        ImageFormat::R32Sint => wgpu::TextureFormat::R32Sint,
        ImageFormat::R8G8B8A8Uint => wgpu::TextureFormat::Rgba8Uint,
        ImageFormat::R8G8B8A8Sint => wgpu::TextureFormat::Rgba8Sint,
        ImageFormat::R8G8B8Unorm => wgpu::TextureFormat::Rgba8Unorm,
        ImageFormat::R8G8B8A8Unorm => wgpu::TextureFormat::Rgba8Unorm,
        ImageFormat::D32 => wgpu::TextureFormat::Depth32Float,
    }
}

pub fn convert_attribute_format(format: AttributeFormat) -> wgpu::VertexFormat {
    match format {
        AttributeFormat::Uint32 => wgpu::VertexFormat::Uint32,
        AttributeFormat::Float32_32 => wgpu::VertexFormat::Float32x2,
        AttributeFormat::Float32_32_32 => wgpu::VertexFormat::Float32x3,
        AttributeFormat::Float32_32_32_32 => wgpu::VertexFormat::Float32x4,
    }
}

pub fn convert_shader_stage(stage: ShaderStage) -> wgpu::ShaderStages {
    match stage {
        ShaderStage::Compute => wgpu::ShaderStages::COMPUTE,
        ShaderStage::Vertex => wgpu::ShaderStages::VERTEX,
        ShaderStage::Pixel => wgpu::ShaderStages::FRAGMENT,
    }
}
