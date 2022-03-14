use std::sync::Arc;

use sjgfx_interface::{VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo};

use crate::DeviceVk;

pub struct VertexStateVk {
    attribute_state_infos: Arc<Vec<VertexAttributeStateInfo>>,
    buffer_state_infos: Arc<Vec<VertexBufferStateInfo>>,
}

impl VertexStateVk {
    pub fn new(_device: &DeviceVk, info: &VertexStateInfo) -> Self {
        Self {
            attribute_state_infos: Arc::new(info.get_attribute_state_info_array().to_vec()),
            buffer_state_infos: Arc::new(info.get_buffer_state_info_array().to_vec()),
        }
    }

    pub fn get_attribute_state_infos(&self) -> &[VertexAttributeStateInfo] {
        &self.attribute_state_infos
    }

    pub fn clone_attribute_state_infos(&self) -> Arc<Vec<VertexAttributeStateInfo>> {
        self.attribute_state_infos.clone()
    }

    pub fn clone_buffer_state_infos(&self) -> Arc<Vec<VertexBufferStateInfo>> {
        self.buffer_state_infos.clone()
    }
}
