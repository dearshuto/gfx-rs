use super::super::blend_state_api::{BlendStateInfo, IBlendStateImpl};
use super::super::Device;

pub struct BlendStateImpl {
    _pipeline_color_blend_state_create_info: ash::vk::PipelineColorBlendStateCreateInfo,
}

impl IBlendStateImpl for BlendStateImpl {
    fn new(_device: &Device, _info: &BlendStateInfo) -> Self {
        let color_blend_attachment_states = [ash::vk::PipelineColorBlendAttachmentState::builder()
            .blend_enable(true)
            .src_color_blend_factor(ash::vk::BlendFactor::SRC_COLOR)
            .dst_color_blend_factor(ash::vk::BlendFactor::ONE_MINUS_DST_COLOR)
            .color_blend_op(ash::vk::BlendOp::ADD)
            .src_alpha_blend_factor(ash::vk::BlendFactor::ZERO)
            .dst_alpha_blend_factor(ash::vk::BlendFactor::ZERO)
            .alpha_blend_op(ash::vk::BlendOp::ADD)
            .color_write_mask(ash::vk::ColorComponentFlags::all())
            .build()];

        Self {
            _pipeline_color_blend_state_create_info:
                ash::vk::PipelineColorBlendStateCreateInfo::builder()
                    .logic_op(ash::vk::LogicOp::CLEAR)
                    .attachments(&color_blend_attachment_states)
                    .build(),
        }
    }
}
