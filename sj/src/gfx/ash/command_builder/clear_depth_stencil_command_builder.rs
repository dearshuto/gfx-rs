use ash::version::DeviceV1_0;

use crate::gfx::{texture_api::TextureArrayRange, DepthStencilClearMode, DepthStencilView, Device};

pub struct ClearDepthStencilCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _image: ash::vk::Image,
    _clear_depth_steuci_value: ash::vk::ClearDepthStencilValue,
}

impl<'a> ClearDepthStencilCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        depth_stencil: &mut DepthStencilView,
        depth: f32,
        stencil: i32,
        _clear_mode: &DepthStencilClearMode,
        _texture_array_range: Option<&TextureArrayRange>,
    ) -> Self {
        let clear_depth_stencil_value = ash::vk::ClearDepthStencilValue::builder()
            .depth(depth)
            .stencil(stencil as u32)
            .build();

        Self {
            _device: device,
            _command_buffer: command_buffer,
            _image: depth_stencil.to_data().get_image(),
            _clear_depth_steuci_value: clear_depth_stencil_value,
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();

        unsafe {
            let image_memory_bariier = ash::vk::ImageMemoryBarrier::builder()
                .src_access_mask(
                    ash::vk::AccessFlags::MEMORY_READ | ash::vk::AccessFlags::MEMORY_WRITE,
                )
                .dst_access_mask(ash::vk::AccessFlags::TRANSFER_WRITE)
                .old_layout(ash::vk::ImageLayout::UNDEFINED)
                .new_layout(ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .src_queue_family_index(0)
                .dst_queue_family_index(0)
                .image(self._image)
                .subresource_range(
                    ash::vk::ImageSubresourceRange::builder()
                        .aspect_mask(ash::vk::ImageAspectFlags::DEPTH)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1)
                        .build(),
                )
                .build();
            device_ash.cmd_pipeline_barrier(
                self._command_buffer,
                ash::vk::PipelineStageFlags::VERTEX_SHADER
                    | ash::vk::PipelineStageFlags::FRAGMENT_SHADER,
                ash::vk::PipelineStageFlags::TRANSFER,
                ash::vk::DependencyFlags::empty(),
                &[], // memory_barrier
                &[], // buffer_memory_barriier
                &[image_memory_bariier],
            );
        }

        unsafe {
            device_ash.cmd_clear_depth_stencil_image(
                self._command_buffer,
                self._image,
                ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &self._clear_depth_steuci_value,
                &[ash::vk::ImageSubresourceRange::builder()
                    .aspect_mask(ash::vk::ImageAspectFlags::DEPTH)
                    .base_array_layer(0)
                    .base_mip_level(0)
                    .layer_count(1)
                    .level_count(1)
                    .build()],
            );

            let image_memory_bariier = ash::vk::ImageMemoryBarrier::builder()
                .src_access_mask(ash::vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(ash::vk::AccessFlags::SHADER_READ)
                .old_layout(ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .src_queue_family_index(0)
                .dst_queue_family_index(0)
                .image(self._image)
                .subresource_range(
                    ash::vk::ImageSubresourceRange::builder()
                        .aspect_mask(ash::vk::ImageAspectFlags::DEPTH)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1)
                        .build(),
                )
                .build();
            device_ash.cmd_pipeline_barrier(
                self._command_buffer,
                ash::vk::PipelineStageFlags::TRANSFER,
                ash::vk::PipelineStageFlags::VERTEX_SHADER,
                ash::vk::DependencyFlags::empty(),
                &[], // memory_barrier
                &[], // buffer_memory_barriier
                &[image_memory_bariier],
            );
        }
    }
}
