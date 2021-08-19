use crate::gfx::{VertexAttributeStateInfo, VertexBufferStateInfo};

use super::super::vertex_state_api::{IVertexState, VertexStateInfo};
use super::super::Device;

pub struct VertexStateWgpu {
    _vertex_attribute_state_info: Vec<VertexAttributeStateInfo>,
    _vertex_buffer_state_info: Vec<VertexBufferStateInfo>,
}

impl IVertexState for VertexStateWgpu {
    fn new(_device: &Device, info: &VertexStateInfo) -> Self {
        let vertex_attribute_state_info_arra = info.get_attribute_state_info_array().to_vec();
        let vertex_buffer_state_info_array = info.get_buffer_state_info_array().to_vec();

        Self {
            _vertex_attribute_state_info: vertex_attribute_state_info_arra,
            _vertex_buffer_state_info: vertex_buffer_state_info_array,
        }
    }
}
