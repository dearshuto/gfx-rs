use bitflags::bitflags;

bitflags! {
    pub struct GpuAccess: u32 {
        const VERTEX_BUFFER = 1;
        const INDEX_BUFFER = 1 << 1;
        const CONSTANT_BUFFER = 1 << 2;
        const TEXTURE = 0x08;
        const UNORDERED_ACCESS_BUFFER = 1 << 3;
        const COLOR_BUFFER = 1 << 4;
        const DEPTH_STENCIL = 1 << 5;
        const READ = 1 << 6;
        const WRITE = 1 << 7;
        const INDIRECT_BUFFER = 1 << 8;
        const IMAGE = 1 << 9;
    }
}

#[derive(Clone)]
pub enum DebugMode {
    Full,
    FullAssertion,
}

#[derive(Clone)]
pub enum ImageFormat {
    R8Unorm,
    R8Snorm,
    R8Uint,
    R8Sint,
    R32Uint,
    R32Sint,
    R8G8B8A8Sint,
    R8G8B8A8Uint,
    R8G8B8A8Unorm,
    R8G8B8Unorm,
    D32,
}

#[derive(Clone, PartialEq)]
pub enum PrimitiveTopology {
    PointList,
    TriangleList,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttributeFormat {
    Uint32,
    Float32_32,
    Float32_32_32,
    Float32_32_32_32,
}

#[derive(Clone, PartialEq)]
pub enum IndexFormat {
    Uint32,
}

#[derive(Clone)]
pub enum ShaderStage {
    Vertex,
    Pixel,
    Compute,
}
