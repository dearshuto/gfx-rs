use std::sync::Arc;

use sjgfx_interface::{IVertexState, VertexBufferStateInfo, VertexStateInfo};
use wgpu::{VertexAttribute, VertexBufferLayout, VertexStepMode};

use crate::DeviceWgpu;

pub struct VertexStateWgpu {
    vertex_buffer_state_info: Arc<Vec<VertexBufferStateInfo>>,
}

impl VertexStateWgpu {
    pub fn new(_device: &DeviceWgpu, info: &VertexStateInfo) -> Self {
        Self {
            vertex_buffer_state_info: Arc::new(info.get_buffer_state_info_array().to_vec()),
        }
    }

    pub fn view(&self) -> VertexStateView {
        VertexStateView::new(self)
    }

    pub fn clone_vertex_buffer_state_infos(&self) -> Arc<Vec<VertexBufferStateInfo>> {
        self.vertex_buffer_state_info.clone()
    }
}

pub struct VertexStateView {
    vertex_buffer_state_info: Arc<Vec<VertexBufferStateInfo>>,
}

impl VertexStateView {
    pub fn new(vertex_state: &VertexStateWgpu) -> Self {
        Self {
            vertex_buffer_state_info: vertex_state.clone_vertex_buffer_state_infos(),
        }
    }

    pub fn create_vertex_buffer_layout<'a>(
        &self,
        vertex_attributes: &'a [VertexAttribute],
    ) -> Vec<VertexBufferLayout<'a>> {
        if self.vertex_buffer_state_info.is_empty() {
            vec![]
        } else {
            let vertex_buffer_layout = VertexBufferLayout {
                array_stride: self.vertex_buffer_state_info[0].get_stride() as u64,
                step_mode: VertexStepMode::Vertex,
                attributes: vertex_attributes,
            };

            vec![vertex_buffer_layout]
        }
    }
}

impl IVertexState for VertexStateWgpu {
    type DeviceType = DeviceWgpu;

    fn new(device: &Self::DeviceType, info: &VertexStateInfo) -> Self {
        Self::new(device, info)
    }
}
