use super::super::super::{Buffer, BufferTextureCopyRegion, Device, Texture};
use ash::version::DeviceV1_0;

pub struct CopyImageToBufferCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _image: ash::vk::Image,
    _image_layout: ash::vk::ImageLayout,
    _buffer: ash::vk::Buffer,
    _buffer_offset: u64,
    _buffer_row_length: u32,
    _buffer_image_height: u32,
    _image_offset: ash::vk::Offset3D,
    _image_extent: ash::vk::Extent3D,
    _image_subresource_layers: ash::vk::ImageSubresourceLayers,
}

impl<'a> CopyImageToBufferCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        dst_buffer: &Buffer,
        src_texture: &Texture,
        copy_region: &BufferTextureCopyRegion,
    ) -> Self {
        let texture_subresource = copy_region
            .get_texture_copy_region()
            .get_texture_subresource();
        Self {
            _device: device,
            _command_buffer: command_buffer,
            _image: *src_texture.to_data().get_image(),
            _image_layout: ash::vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            _buffer: dst_buffer.to_data().get_buffer(),
            _buffer_offset: copy_region.get_offset() as u64,
            _buffer_row_length: copy_region.get_image_width() as u32,
            _buffer_image_height: copy_region.get_image_height() as u32,
            _image_offset: ash::vk::Offset3D::builder()
                .x(copy_region.get_texture_copy_region().get_offset_u())
                .y(copy_region.get_texture_copy_region().get_offset_v())
                .z(copy_region.get_texture_copy_region().get_offset_w())
                .build(),
            _image_extent: ash::vk::Extent3D::builder()
                .width(copy_region.get_texture_copy_region().get_width() as u32)
                .height(copy_region.get_texture_copy_region().get_height() as u32)
                .depth(copy_region.get_texture_copy_region().get_depth() as u32)
                .build(),
            _image_subresource_layers: ash::vk::ImageSubresourceLayers::builder()
                .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                .mip_level(texture_subresource.get_mip_level() as u32)
                .base_array_layer(texture_subresource.get_array_index() as u32)
                .layer_count(1)
                .build(),
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();
        let regions = [ash::vk::BufferImageCopy::builder()
            .buffer_offset(self._buffer_offset)
            .buffer_row_length(self._buffer_row_length)
            .buffer_image_height(self._buffer_image_height)
            .image_offset(self._image_offset)
            .image_extent(self._image_extent)
            .image_subresource(self._image_subresource_layers)
            .build()];

        unsafe {
            device_ash.cmd_copy_image_to_buffer(
                self._command_buffer,
                self._image,
                self._image_layout,
                self._buffer,
                &regions,
            );
        }
    }
}
