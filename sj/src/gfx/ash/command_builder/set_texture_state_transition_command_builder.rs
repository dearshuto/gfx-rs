use crate::gfx::{Device, Texture, TextureSubresourceRange};
use ash::version::DeviceV1_0;

pub struct SetTextureStateTransitionCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _image: ash::vk::Image,
}

impl<'a> SetTextureStateTransitionCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        texture: &Texture,
        range: TextureSubresourceRange,
        old_state: TextureState,
        old_stage_bit: PipelineStageBit,
        new_state: TextureState,
        new_stage_bit: PipelineStageBit,
    ) -> Self {
        Self {
            _device: device,
            _command_buffer: command_buffer,
            _iamge: texture.to_data().get_image(),
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();
        let image_memory_bariier = ash::vk::ImageMemoryBarrier::builder()
            .src_access_mask(ash::vk::AccessFlags::TRANSFER_READ)
            .dst_access_mask(ash::vk::AccessFlags::TRANSFER_WRITE)
            .old_layout(ash::vk::ImageLayout::UNDEFINED)
            .new_layout(ash::vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .src_queue_family_index(0)
            .dst_queue_family_index(0)
            .image(self._image)
            .subresource_range(
                ash::vk::ImageSubresourceRange::builder()
                    .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build(),
            )
            .build();

        unsafe {
            device_ash.cmd_pipeline_barrier(
                self._command_buffer,
                ash::vk::PipelineStageFlags::TRANSFER,
                ash::vk::PipelineStageFlags::TRANSFER,
                ash::vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_bariier],
            );
        }
    }
}
