use sjgfx_interface::{IVertexState, VertexAttributeStateInfo, VertexBufferStateInfo};

use crate::DeviceWsys;

pub struct VertexStateWsys {
    vertex_attribute_state_infos: Vec<VertexAttributeStateInfo>,
    vertex_buffer_state_infos: Vec<VertexBufferStateInfo>,
}

impl VertexStateWsys {
    pub fn get_attribute_state_infos(&self) -> &[VertexAttributeStateInfo] {
        &self.vertex_attribute_state_infos
    }

    pub fn get_buffer_state_infos(&self) -> &[VertexBufferStateInfo] {
        &self.vertex_buffer_state_infos
    }
}

impl IVertexState for VertexStateWsys {
    type DeviceType = DeviceWsys;

    fn new(_device: &Self::DeviceType, info: &sjgfx_interface::VertexStateInfo) -> Self {
        Self {
            vertex_attribute_state_infos: info.get_attribute_state_info_array().to_vec(),
            vertex_buffer_state_infos: info.get_buffer_state_info_array().to_vec(),
        }
    }
}
