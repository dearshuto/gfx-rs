use ash::version::DeviceV1_0;

use crate::gfx::{
    BlendStateInfo, BlendTargetStateInfo, DepthStencilStateInfo, RasterizerStateInfo,
    VertexStateInfo,
};

use super::super::pipeline_api::{ComputePipelineInfo, GraphicsPipelineInfo, IPipelineImpl};
use super::super::{Device, Shader, VertexAttributeStateInfo, VertexBufferStateInfo};

pub struct PipelineImpl<'a> {
    _device: &'a Device,
    _shader: &'a Shader<'a>,
    _pipeline: ash::vk::Pipeline,
}

impl<'a> PipelineImpl<'a> {
    pub fn get_shader(&self) -> &'a Shader<'a> {
        self._shader
    }

    pub fn get_pipeline(&self) -> &ash::vk::Pipeline {
        &self._pipeline
    }
}

impl<'a> IPipelineImpl<'a> for PipelineImpl<'a> {
    fn new_as_graphics(device: &'a Device, info: &'a GraphicsPipelineInfo) -> Self {
        let shader_entry_name = std::ffi::CString::new("main").unwrap();
        let device_ash = device.to_data().get_device();
        let shader = info.get_shader().to_data();
        let vertex_shader_module = shader.get_vertex_shader_module();
        let pixel_shader_module = shader.get_pixel_shader_module();

        let vertex_stage_create_info = ash::vk::PipelineShaderStageCreateInfo::builder()
            .module(*vertex_shader_module)
            .name(&shader_entry_name)
            .stage(ash::vk::ShaderStageFlags::VERTEX)
            .build();
        let pixel_stage_create_info = ash::vk::PipelineShaderStageCreateInfo::builder()
            .module(*pixel_shader_module)
            .name(&shader_entry_name)
            .stage(ash::vk::ShaderStageFlags::FRAGMENT)
            .build();

        // TODO: レンダーパスをどうやって生成するのか
        let render_pass_create_info = ash::vk::RenderPassCreateInfo::builder().build();

        unsafe {
            let render_pass = device_ash
                .create_render_pass(&&render_pass_create_info, None)
                .unwrap();

            let graphics_pipeline_create_info = ash::vk::GraphicsPipelineCreateInfo::builder()
                .stages(&[vertex_stage_create_info, pixel_stage_create_info])
                .vertex_input_state(&info.get_vertex_state_info().as_ash())
                .rasterization_state(&info.get_rasterizer_state().as_ash())
                .depth_stencil_state(&info.get_depth_stencil_state().as_ash())
                .color_blend_state(&info.get_blend_state_info().as_ash())
                .multisample_state(&info.create_pipeline_multi_sample_create_info())
                .dynamic_state(&info.create_pipeline_dynamic_state_create_info())
                .render_pass(render_pass)
                .layout(*shader.get_pipeline_layout())
                .build();

            let pipelines = device_ash
                .create_graphics_pipelines(
                    ash::vk::PipelineCache::null(),
                    &[graphics_pipeline_create_info],
                    None,
                )
                .unwrap();

            Self {
                _device: device,
                _shader: info.get_shader(),
                _pipeline: pipelines[0],
            }
        }
    }

    fn new_as_compute<'b>(device: &'a Device, info: ComputePipelineInfo<'a>) -> Self {
        let shader_entry_name = std::ffi::CString::new("main").unwrap();
        let device_ash = device.to_data().get_device();
        let shader_impl = info.get_shader().to_data();

        let shader_stage_create_info = ash::vk::PipelineShaderStageCreateInfo::builder()
            .module(*shader_impl.get_compute_shader_modules())
            .name(&shader_entry_name)
            .stage(ash::vk::ShaderStageFlags::COMPUTE)
            .build();
        let compute_pipeline_create_info = ash::vk::ComputePipelineCreateInfo::builder()
            .stage(shader_stage_create_info)
            .layout(*shader_impl.get_pipeline_layout())
            .build();

        unsafe {
            let pipelines = device_ash
                .create_compute_pipelines(
                    ash::vk::PipelineCache::null(),
                    &[compute_pipeline_create_info],
                    None,
                )
                .unwrap();

            Self {
                _device: device,
                _shader: info.get_shader(),
                _pipeline: pipelines[0],
            }
        }
    }
}

impl<'a> GraphicsPipelineInfo<'a> {
    pub fn create_pipeline_multi_sample_create_info(
        &self,
    ) -> ash::vk::PipelineMultisampleStateCreateInfo {
        ash::vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(ash::vk::SampleCountFlags::TYPE_1)
            .build()
    }

    pub fn create_pipeline_dynamic_state_create_info(
        &self,
    ) -> ash::vk::PipelineDynamicStateCreateInfo {
        let dynamic_state = [
            ash::vk::DynamicState::VIEWPORT,
            ash::vk::DynamicState::SCISSOR,
        ];

        ash::vk::PipelineDynamicStateCreateInfo::builder()
            .dynamic_states(&dynamic_state)
            .build()
    }
}

impl VertexAttributeStateInfo {
    pub fn as_ash(&self) -> ash::vk::VertexInputAttributeDescription {
        ash::vk::VertexInputAttributeDescription::builder()
            .binding(self.get_buffer_index() as u32)
            .format(self.get_format_as_ash())
            .location(self.get_slot() as u32)
            .offset(self.get_offset() as u32)
            .build()
    }

    fn get_format_as_ash(&self) -> ash::vk::Format {
        match self.get_format() {
            crate::gfx::AttributeFormat::Float32_32 => ash::vk::Format::R32G32_SFLOAT,
            crate::gfx::AttributeFormat::Float32_32_32 => ash::vk::Format::R32G32B32_SFLOAT,
        }
    }
}

impl VertexBufferStateInfo {
    pub fn as_ash(&self, index: i32) -> ash::vk::VertexInputBindingDescription {
        ash::vk::VertexInputBindingDescription::builder()
            .binding(index as u32)
            .stride(self.get_stride() as u32)
            .input_rate(ash::vk::VertexInputRate::VERTEX)
            .build()
    }
}

impl<'a> VertexStateInfo<'a> {
    pub fn as_ash(&self) -> ash::vk::PipelineVertexInputStateCreateInfo {
        let vertex_attribute_descriptions: Vec<ash::vk::VertexInputAttributeDescription> = self
            .get_attribute_state_info_array()
            .iter()
            .map(|info| info.as_ash())
            .collect();

        let vertex_bindings_descriptions: Vec<ash::vk::VertexInputBindingDescription> = self
            .get_buffer_state_info_array()
            .iter()
            .enumerate()
            .map(|(index, ref info)| info.as_ash(index as i32))
            .collect();

        ash::vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(&vertex_attribute_descriptions)
            .vertex_binding_descriptions(&&vertex_bindings_descriptions)
            .build()
    }
}

impl RasterizerStateInfo {
    pub fn as_ash(&self) -> ash::vk::PipelineRasterizationStateCreateInfo {
        ash::vk::PipelineRasterizationStateCreateInfo::builder()
            .cull_mode(ash::vk::CullModeFlags::NONE)
            .polygon_mode(ash::vk::PolygonMode::FILL)
            .front_face(ash::vk::FrontFace::COUNTER_CLOCKWISE)
            .build()
    }
}

impl DepthStencilStateInfo {
    pub fn as_ash(&self) -> ash::vk::PipelineDepthStencilStateCreateInfo {
        ash::vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(self.is_depth_test_enabled())
            .depth_write_enable(self.is_depth_write_enabled())
            .depth_compare_op(ash::vk::CompareOp::LESS_OR_EQUAL)
            .build()
    }
}

impl<'a> BlendStateInfo<'a> {
    pub fn as_ash(&self) -> ash::vk::PipelineColorBlendStateCreateInfo {
        let attachments: Vec<ash::vk::PipelineColorBlendAttachmentState> = self
            .get_color_target_blend_state_info()
            .iter()
            .map(BlendTargetStateInfo::as_ash)
            .collect();

        ash::vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op(ash::vk::LogicOp::CLEAR)
            .attachments(&attachments)
            .build()
    }
}

impl BlendTargetStateInfo {
    pub fn as_ash(&self) -> ash::vk::PipelineColorBlendAttachmentState {
        ash::vk::PipelineColorBlendAttachmentState::builder()
            .blend_enable(true)
            .src_color_blend_factor(ash::vk::BlendFactor::SRC_COLOR)
            .dst_color_blend_factor(ash::vk::BlendFactor::ONE_MINUS_DST_COLOR)
            .color_blend_op(ash::vk::BlendOp::ADD)
            .src_alpha_blend_factor(ash::vk::BlendFactor::ZERO)
            .dst_alpha_blend_factor(ash::vk::BlendFactor::ZERO)
            .alpha_blend_op(ash::vk::BlendOp::ADD)
            .color_write_mask(ash::vk::ColorComponentFlags::all())
            .build()
    }
}

impl<'a> Drop for PipelineImpl<'a> {
    fn drop(&mut self) {
        let device_ash = self._device.to_data().get_device();
        unsafe {
            device_ash.destroy_pipeline(self._pipeline, None);
        }
    }
}
