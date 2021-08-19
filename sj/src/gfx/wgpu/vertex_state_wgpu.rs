use crate::gfx::{VertexAttributeStateInfo, VertexBufferStateInfo};

use super::super::vertex_state_api::{IVertexState, VertexStateInfo};
use super::super::Device;

pub struct VertexStateWgpu {
    _vertex_attribute_state_info_array: Vec<VertexAttributeStateInfo>,
    _vertex_buffer_state_info_array: Vec<VertexBufferStateInfo>,
}

impl IVertexState for VertexStateWgpu {
    fn new(_device: &Device, info: &VertexStateInfo) -> Self {
        let vertex_attribute_state_info_arra = info.get_attribute_state_info_array().to_vec();
        let vertex_buffer_state_info_array = info.get_buffer_state_info_array().to_vec();

        Self {
            _vertex_attribute_state_info_array: vertex_attribute_state_info_arra,
            _vertex_buffer_state_info_array: vertex_buffer_state_info_array,
        }
    }
}

impl VertexStateWgpu {
    pub fn get_vertex_attribute_state_info_array(&self) -> &[VertexAttributeStateInfo] {
        &self._vertex_attribute_state_info_array
    }

    pub fn get_vertex_buffer_state_info_array(&self) -> &[VertexBufferStateInfo] {
        &self._vertex_buffer_state_info_array
    }
}
