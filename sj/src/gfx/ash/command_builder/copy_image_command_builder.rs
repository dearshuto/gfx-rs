use crate::gfx::{texture_api::TextureSubresource, Device, Texture, TextureCopyRegion};
use ash::version::DeviceV1_0;

pub struct CopyImageCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _src_image: ash::vk::Image,
    _dst_image: ash::vk::Image,
    _image_copy: [ash::vk::ImageCopy; 1],
}

impl<'a> CopyImageCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        dst_texture: &mut Texture,
        dst_subresource: &TextureSubresource,
        dst_offset_u: i32,
        dst_offset_v: i32,
        dst_offset_w: i32,
        src_texture: &Texture,
        src_copy_range: TextureCopyRegion,
    ) -> Self {
        let image_copy = [ash::vk::ImageCopy::builder()
            .src_offset(
                ash::vk::Offset3D::builder()
                    .x(src_copy_range.get_offset_u())
                    .y(src_copy_range.get_offset_v())
                    .z(src_copy_range.get_offset_w())
                    .build(),
            )
            .src_subresource(src_copy_range.get_texture_subresource().to_ash())
            .dst_offset(
                ash::vk::Offset3D::builder()
                    .x(dst_offset_u)
                    .y(dst_offset_v)
                    .z(dst_offset_w)
                    .build(),
            )
            .dst_subresource(dst_subresource.to_ash())
            .extent(src_copy_range.to_extent())
            .build()];

        Self {
            _device: device,
            _command_buffer: command_buffer,
            _src_image: *src_texture.to_data().get_image(),
            _dst_image: *dst_texture.to_data().get_image(),
            _image_copy: image_copy,
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();
        unsafe {
            device_ash.cmd_copy_image(
                self._command_buffer,
                self._src_image,
                ash::vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                self._dst_image,
                ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &self._image_copy,
            );
        }
    }
}

impl TextureCopyRegion {
    pub fn to_extent(&self) -> ash::vk::Extent3D {
        ash::vk::Extent3D::builder()
            .width(self.get_width() as u32)
            .height(self.get_height() as u32)
            .depth(self.get_depth() as u32)
            .build()
    }
}

impl TextureSubresource {
    pub fn to_ash(&self) -> ash::vk::ImageSubresourceLayers {
        ash::vk::ImageSubresourceLayers::builder()
            .mip_level(self.get_mip_level() as u32)
            .base_array_layer(self.get_array_index() as u32)
            .aspect_mask(ash::vk::ImageAspectFlags::COLOR) // TODO
            .layer_count(1)
            .build()
    }
}
