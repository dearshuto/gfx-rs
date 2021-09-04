use super::VkAutoCommandBufferBuilder;

pub struct CopyImageToBufferCommandBuilder {}

impl CopyImageToBufferCommandBuilder {
    pub fn new(
        dst_buffer: &mut crate::gfx::Buffer,
        src_texture: &crate::gfx::Texture,
        copy_region: &crate::gfx::BufferTextureCopyRegion,
    ) -> Self {
        Self {}
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        command_builder
    }
}
