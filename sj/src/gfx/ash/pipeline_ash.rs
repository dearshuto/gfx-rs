use ash::version::DeviceV1_0;

use crate::gfx::{
    BlendStateInfo, BlendTargetStateInfo, DepthStencilStateInfo, RasterizerStateInfo,
    VertexStateInfo,
};

use super::super::pipeline_api::{ComputePipelineInfo, GraphicsPipelineInfo, IPipelineImpl};
use super::super::{Device, Shader, VertexAttributeStateInfo, VertexBufferStateInfo};

enum PipelineType {
    Graphics,
    Compute,
}

pub struct PipelineImpl<'a> {
    _device: &'a Device,
    _shader: &'a Shader<'a>,
    _compute_pipeline: Option<ash::vk::Pipeline>,
    _vertex_input_attribute_desctioption_array:
        Option<Vec<ash::vk::VertexInputAttributeDescription>>,
    _blend_attachment_state_array: Option<Vec<ash::vk::PipelineColorBlendAttachmentState>>,
    _blend_state_create_info: Option<ash::vk::PipelineColorBlendStateCreateInfo>,
    _vertex_input_binding_description_array: Option<Vec<ash::vk::VertexInputBindingDescription>>,
    _rasterization_state_create_info: Option<ash::vk::PipelineRasterizationStateCreateInfo>,
    _depth_stencil_state_create_info: Option<ash::vk::PipelineDepthStencilStateCreateInfo>,
    _pipeline_type: PipelineType,
}

impl<'a> PipelineImpl<'a> {
    pub fn get_shader(&self) -> &'a Shader<'a> {
        self._shader
    }

    pub fn get_pipeline(&self) -> &ash::vk::Pipeline {
        &self._compute_pipeline.as_ref().unwrap()
    }

    pub fn create_graphics_pipeline(&self, render_pass: ash::vk::RenderPass) -> ash::vk::Pipeline {
        let device_ash = self._device.to_data().get_device();
        let shader_entry_name = std::ffi::CString::new("main").unwrap();

        let vertex_shader_module = self._shader.to_data().get_vertex_shader_module();
        let pixel_shader_module = self._shader.to_data().get_pixel_shader_module();

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

        let vertex_input_state_create_info = ash::vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(
                self._vertex_input_attribute_desctioption_array
                    .as_ref()
                    .unwrap(),
            )
            .vertex_binding_descriptions(
                self._vertex_input_binding_description_array
                    .as_ref()
                    .unwrap(),
            )
            .build();

        let viewports = [ash::vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: 640.0,
            height: 480.0,
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let scissors = [ash::vk::Rect2D {
            offset: ash::vk::Offset2D { x: 0, y: 0 },
            extent: ash::vk::Extent2D {
                width: 640,
                height: 640,
            },
        }];
        let viewport_state_info = ash::vk::PipelineViewportStateCreateInfo::builder()
            .scissors(&scissors)
            .viewports(&viewports)
            .build();

        let vertex_input_assembly_state_info = ash::vk::PipelineInputAssemblyStateCreateInfo {
            topology: ash::vk::PrimitiveTopology::TRIANGLE_LIST,
            ..Default::default()
        };
        let graphics_pipeline_create_info = ash::vk::GraphicsPipelineCreateInfo::builder()
            .stages(&[vertex_stage_create_info, pixel_stage_create_info])
            .viewport_state(&viewport_state_info)
            .vertex_input_state(&vertex_input_state_create_info)
            .rasterization_state(&self._rasterization_state_create_info.unwrap())
            .depth_stencil_state(&self._depth_stencil_state_create_info.unwrap())
            .color_blend_state(&self._blend_state_create_info.unwrap())
            .multisample_state(&self.create_pipeline_multi_sample_create_info())
            .dynamic_state(&self.create_pipeline_dynamic_state_create_info())
            .render_pass(render_pass)
            .input_assembly_state(&vertex_input_assembly_state_info)
            .layout(*self._shader.to_data().get_pipeline_layout())
            .build();

        unsafe {
            let pipelines = device_ash
                .create_graphics_pipelines(
                    ash::vk::PipelineCache::null(),
                    &[graphics_pipeline_create_info],
                    None,
                )
                .unwrap();
            pipelines[0]
        }
    }

    pub fn is_graphics_pipeline(&self) -> bool {
        match self._pipeline_type {
            PipelineType::Graphics => true,
            PipelineType::Compute => false,
        }
    }

    pub fn is_compute_pipeline(&self) -> bool {
        !self.is_graphics_pipeline()
    }

    fn create_pipeline_multi_sample_create_info(
        &self,
    ) -> ash::vk::PipelineMultisampleStateCreateInfo {
        ash::vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(ash::vk::SampleCountFlags::TYPE_1)
            .build()
    }

    fn create_pipeline_dynamic_state_create_info(&self) -> ash::vk::PipelineDynamicStateCreateInfo {
        let dynamic_state = [
            ash::vk::DynamicState::VIEWPORT,
            ash::vk::DynamicState::SCISSOR,
        ];

        ash::vk::PipelineDynamicStateCreateInfo::builder()
            .dynamic_states(&dynamic_state)
            .build()
    }
}

impl<'a> IPipelineImpl<'a> for PipelineImpl<'a> {
    fn new_as_graphics(device: &'a Device, info: &'a GraphicsPipelineInfo) -> Self {
        let vertex_attribute_descriptions: Vec<ash::vk::VertexInputAttributeDescription> = info
            .get_vertex_state_info()
            .get_attribute_state_info_array()
            .iter()
            .map(|info| info.as_ash())
            .collect();

        let vertex_bindings_descriptions: Vec<ash::vk::VertexInputBindingDescription> = info
            .get_vertex_state_info()
            .get_buffer_state_info_array()
            .iter()
            .enumerate()
            .map(|(index, ref info)| info.as_ash(index as i32))
            .collect();
        let (blend_state_create_info, attachment) = info.get_blend_state_info().as_ash();

        Self {
            _device: device,
            _shader: info.get_shader(),
            _compute_pipeline: None,
            _vertex_input_binding_description_array: Some(vertex_bindings_descriptions),
            _vertex_input_attribute_desctioption_array: Some(vertex_attribute_descriptions),
            _rasterization_state_create_info: Some(info.get_rasterizer_state().as_ash()),
            _depth_stencil_state_create_info: Some(info.get_depth_stencil_state().as_ash()),
            _blend_attachment_state_array: Some(attachment),
            _blend_state_create_info: Some(blend_state_create_info),
            _pipeline_type: PipelineType::Graphics,
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
                _compute_pipeline: Some(pipelines[0]),
                _vertex_input_attribute_desctioption_array: None,
                _vertex_input_binding_description_array: None,
                _rasterization_state_create_info: None,
                _depth_stencil_state_create_info: None,
                _blend_attachment_state_array: None,
                _blend_state_create_info: None,
                _pipeline_type: PipelineType::Compute,
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
            ash::vk::DynamicState::LINE_WIDTH,
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
            .line_width(1.0)
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
    pub fn as_ash(
        &self,
    ) -> (
        ash::vk::PipelineColorBlendStateCreateInfo,
        Vec<ash::vk::PipelineColorBlendAttachmentState>,
    ) {
        let attachments: Vec<ash::vk::PipelineColorBlendAttachmentState> = self
            .get_color_target_blend_state_info()
            .iter()
            .map(BlendTargetStateInfo::as_ash)
            .collect();

        let color_blend_state = ash::vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op(ash::vk::LogicOp::CLEAR)
            .attachments(&attachments)
            .build();

        (color_blend_state, attachments)
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
        if self.is_compute_pipeline() {
            let device_ash = self._device.to_data().get_device();

            unsafe {
                device_ash.destroy_pipeline(self._compute_pipeline.unwrap(), None);
            }
        }
    }
}
