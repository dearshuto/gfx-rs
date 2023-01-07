use sjgfx_interface::{AttributeFormat, IVertexState, VertexStateInfo};
use vulkano::pipeline::graphics::vertex_input::{
    VertexDefinition, VertexInputAttributeDescription, VertexInputBindingDescription,
    VertexInputRate, VertexInputState,
};

use crate::DeviceVk;

pub struct VertexStateVk {
    vertex_input_state: VertexInputState,
}

impl VertexStateVk {
    pub fn new(_device: &DeviceVk, info: &VertexStateInfo) -> Self {
        // 頂点アトリビュートステート
        let attributes = info
            .get_attribute_state_info_array()
            .iter()
            .enumerate()
            .map(|(index, x)| {
                let description = VertexInputAttributeDescription {
                    binding: x.get_buffer_index() as u32,
                    format: Self::convert_format(x.get_format().clone()),
                    offset: x.get_offset() as u32,
                };

                (index as u32, description)
            })
            .collect::<Vec<_>>();

        // 頂点バッファステート
        let bindings = info
            .get_buffer_state_info_array()
            .iter()
            .enumerate()
            .map(|(index, x)| {
                let description = VertexInputBindingDescription {
                    stride: x.get_stride() as u32,
                    input_rate: VertexInputRate::Vertex,
                };
                (index as u32, description)
            })
            .collect::<Vec<_>>();

        let vertex_input_state = VertexInputState::new()
            .attributes(attributes)
            .bindings(bindings);

        Self { vertex_input_state }
    }

    pub(crate) fn view(&self) -> VertexStateView {
        VertexStateView {
            vertex_input_state: self.vertex_input_state.clone(),
        }
    }

    fn convert_format(format: AttributeFormat) -> vulkano::format::Format {
        match format {
            AttributeFormat::Uint32 => vulkano::format::Format::R32_UINT,
            AttributeFormat::Float32_32 => vulkano::format::Format::R32G32_SFLOAT,
            AttributeFormat::Float32_32_32 => vulkano::format::Format::R32G32B32_SFLOAT,
            AttributeFormat::Float32_32_32_32 => vulkano::format::Format::R32G32B32A32_SFLOAT,
        }
    }
}

impl IVertexState for VertexStateVk {
    type DeviceType = DeviceVk;

    fn new(device: &Self::DeviceType, info: &VertexStateInfo) -> Self {
        Self::new(device, info)
    }
}

#[derive(Debug, Clone)]
pub struct VertexStateView {
    vertex_input_state: VertexInputState,
}

unsafe impl VertexDefinition for VertexStateView {
    fn definition(
        &self,
        _interface: &vulkano::shader::ShaderInterface,
    ) -> Result<
        VertexInputState,
        vulkano::pipeline::graphics::vertex_input::IncompatibleVertexDefinitionError,
    > {
        Ok(self.vertex_input_state.clone())
    }
}
