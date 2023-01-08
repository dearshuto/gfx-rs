use sjgfx_interface::{AttributeFormat, IVertexState};

use crate::DeviceAsh;

pub struct VertexStateAsh {
    // 以下ふたつはメモリ上に保持する必要がある
    #[allow(dead_code)]
    vertex_inpute_attribute_descriptions: Vec<ash::vk::VertexInputAttributeDescription>,
    #[allow(dead_code)]
    vertex_binding_descriptions: Vec<ash::vk::VertexInputBindingDescription>,

    vertex_input_state_create_info: ash::vk::PipelineVertexInputStateCreateInfo,
}

impl VertexStateAsh {
    pub fn clone_vertex_input_state_create_info(
        &self,
    ) -> ash::vk::PipelineVertexInputStateCreateInfo {
        self.vertex_input_state_create_info.clone()
    }

    fn create_vertex_inpute_attribute_descriptions(
        info: &sjgfx_interface::VertexStateInfo,
    ) -> Vec<ash::vk::VertexInputAttributeDescription> {
        info.get_attribute_state_info_array()
            .iter()
            .map(|x| ash::vk::VertexInputAttributeDescription {
                location: x.get_slot() as u32,
                binding: x.get_buffer_index() as u32,
                format: Self::convert_attribute_format(x.get_format().clone()),
                offset: x.get_offset() as u32,
            })
            .collect()
    }

    fn create_vertex_binding_descriptions(
        info: &sjgfx_interface::VertexStateInfo,
    ) -> Vec<ash::vk::VertexInputBindingDescription> {
        info.get_buffer_state_info_array()
            .iter()
            .map(|x| -> ash::vk::VertexInputBindingDescription {
                ash::vk::VertexInputBindingDescription {
                    binding: 0, // TODO
                    stride: x.get_stride() as u32,
                    input_rate: ash::vk::VertexInputRate::VERTEX,
                }
            })
            .collect()
    }

    fn convert_attribute_format(attribute_format: AttributeFormat) -> ash::vk::Format {
        match attribute_format {
            AttributeFormat::Uint32 => ash::vk::Format::R32_UINT,
            AttributeFormat::Float32_32 => ash::vk::Format::R32G32_SFLOAT,
            AttributeFormat::Float32_32_32 => ash::vk::Format::R32G32B32_SFLOAT,
            AttributeFormat::Float32_32_32_32 => ash::vk::Format::R32G32B32A32_SFLOAT,
        }
    }
}

impl IVertexState for VertexStateAsh {
    type DeviceType = DeviceAsh;

    fn new(_device: &Self::DeviceType, info: &sjgfx_interface::VertexStateInfo) -> Self {
        let vertex_inpute_attribute_descriptions =
            Self::create_vertex_inpute_attribute_descriptions(info);
        let vertex_binding_descriptions = Self::create_vertex_binding_descriptions(info);

        let vertex_input_state_create_info = ash::vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(&vertex_inpute_attribute_descriptions)
            .vertex_binding_descriptions(&vertex_binding_descriptions)
            .build();

        VertexStateAsh {
            vertex_inpute_attribute_descriptions,
            vertex_binding_descriptions,
            vertex_input_state_create_info,
        }
    }
}
