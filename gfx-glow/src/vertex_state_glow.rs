use sjgfx_interface::{IVertexState, VertexAttributeStateInfo, VertexBufferStateInfo};

use crate::DeviceGlow;

pub struct VertexStateGlow {
    vertex_attribute_state_info_array: Vec<VertexAttributeStateInfo>,
    vertex_buffer_state_info_array: Vec<VertexBufferStateInfo>,
}

impl VertexStateGlow {
    pub fn get_vertex_attribute_state_infos(&self) -> &[VertexAttributeStateInfo] {
        &self.vertex_attribute_state_info_array
    }

    pub fn get_vertex_buffer_state_info_array(&self) -> &[VertexBufferStateInfo] {
        &self.vertex_buffer_state_info_array
    }
}

impl IVertexState for VertexStateGlow {
    type DeviceType = DeviceGlow;

    fn new(_device: &Self::DeviceType, info: &sjgfx_interface::VertexStateInfo) -> Self {
        Self {
            vertex_attribute_state_info_array: info.get_attribute_state_info_array().to_vec(),
            vertex_buffer_state_info_array: info.get_buffer_state_info_array().to_vec(),
        }
    }
}
