use crate::gfx::{Device, PipelineStageBit, Texture, TextureState, TextureSubresourceRange};
use ash::version::DeviceV1_0;

pub struct SetTextureStateTransitionCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _image: ash::vk::Image,
    _old_state: ash::vk::ImageLayout,
    _old_stage_bit: ash::vk::PipelineStageFlags,
    _new_state: ash::vk::ImageLayout,
    _new_stage_bit: ash::vk::PipelineStageFlags,
}

impl<'a> SetTextureStateTransitionCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        texture: &Texture,
        _range: &TextureSubresourceRange,
        old_state: TextureState,
        old_stage_bit: PipelineStageBit,
        new_state: TextureState,
        new_stage_bit: PipelineStageBit,
    ) -> Self {
        Self {
            _device: device,
            _command_buffer: command_buffer,
            _image: *texture.to_data().get_image(),
            _old_state: old_state.to_ash(),
            _old_stage_bit: old_stage_bit.to_ash(),
            _new_state: new_state.to_ash(),
            _new_stage_bit: new_stage_bit.to_ash(),
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();
        let image_memory_bariier = ash::vk::ImageMemoryBarrier::builder()
            .src_access_mask(ash::vk::AccessFlags::all())
            .dst_access_mask(ash::vk::AccessFlags::all())
            .old_layout(self._old_state)
            .new_layout(self._new_state)
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
                self._old_stage_bit,
                self._new_stage_bit,
                ash::vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_bariier],
            );
        }
    }
}

impl TextureState {
    pub fn to_ash(&self) -> ash::vk::ImageLayout {
        match self {
            &TextureState::UNDEFINED => ash::vk::ImageLayout::UNDEFINED,
            &TextureState::COPY_SOURCE => ash::vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            &TextureState::COPY_DESTINATION => ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &TextureState::DATA_TRANSFER => ash::vk::ImageLayout::GENERAL,
            &TextureState::SHADER_READ => ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            _ => todo!(),
        }
    }
}

impl PipelineStageBit {
    pub fn to_ash(&self) -> ash::vk::PipelineStageFlags {
        match self {
            &PipelineStageBit::VERTEX_SHADER => ash::vk::PipelineStageFlags::VERTEX_SHADER,
            _ => ash::vk::PipelineStageFlags::TRANSFER,
        }
    }
}
