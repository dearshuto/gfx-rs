use crate::gfx::vertex_state_api::{
    IVertexState, VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};
use crate::gfx::Device;

pub struct VertexStateVk {
    _vertex_attribute_state_info_array: Vec<VertexAttributeStateInfo>,
    _vertex_buffer_info_array: Vec<VertexBufferStateInfo>,
}

impl IVertexState for VertexStateVk {
    fn new(_device: &Device, info: &VertexStateInfo) -> Self {
        Self {
            _vertex_attribute_state_info_array: info.get_attribute_state_info_array().to_vec(),
            _vertex_buffer_info_array: info.get_buffer_state_info_array().to_vec(),
        }
    }
}
