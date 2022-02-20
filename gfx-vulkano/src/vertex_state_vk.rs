use sjgfx_interface::{VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo};

use crate::DeviceVk;

pub struct VertexStateVk {
    attribute_state_infos: Vec<VertexAttributeStateInfo>,
    buffer_state_infos: Vec<VertexBufferStateInfo>,
}

impl VertexStateVk {
    pub fn new(_device: &DeviceVk, info: &VertexStateInfo) -> Self {
        Self {
            attribute_state_infos: info.get_attribute_state_info_array().to_vec(),
            buffer_state_infos: info.get_buffer_state_info_array().to_vec(),
        }
    }

    pub fn get_attribute_state_infos(&self) -> &[VertexAttributeStateInfo] {
        &self.attribute_state_infos
    }

    pub fn get_buffer_state_infos(&self) -> &[VertexBufferStateInfo] {
        &self.buffer_state_infos
    }
}
