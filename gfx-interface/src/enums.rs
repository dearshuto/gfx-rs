use bitflags::bitflags;

bitflags! {
    pub struct GpuAccess: u32 {
        const VERTEX_BUFFER = 0x01;
        const INDEX_BUFFER = 0x02;
        const CONSTANT_BUFFER = 0x04;
        const TEXTURE = 0x08;
        const UNORDERED_ACCESS_BUFFER = 0x16;
        const COLOR_BUFFER = 0x32;
        const DEPTH_STENCIL = 0x64;
        const READ = 0x128;
        const WRITE = 0x256;
        const INDIRECT_BUFFER = 0x128;
        const IMAGE = 0x4000;
    }
}

#[derive(Clone)]
pub enum ImageFormat {
    R8G8B8A8Unorm,
    D32,
}

#[derive(Clone)]
pub enum PrimitiveTopology {
    PointList,
    TriangleList,
}

#[derive(Clone)]
pub enum AttributeFormat {
    Float32_32,
    Float32_32_32,
}
