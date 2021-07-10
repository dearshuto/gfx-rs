use super::VkAutoCommandBufferBuilder;

pub struct CopyImageCommandBuilder {}

impl CopyImageCommandBuilder {
    pub fn new(
        dst_texture: &mut crate::gfx::Texture,
        dst_subresource: &crate::gfx::TextureSubresource,
        dst_offset_u: i32,
        dst_offset_v: i32,
        dst_offset_w: i32,
        src_texture: &crate::gfx::Texture,
        src_copy_range: crate::gfx::TextureCopyRegion,
    ) -> Self {
        Self {}
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        command_builder
    }
}
