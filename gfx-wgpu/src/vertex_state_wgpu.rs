use sjgfx_interface::{VertexBufferStateInfo, VertexStateInfo};
use wgpu::{VertexAttribute, VertexBufferLayout, VertexStepMode};

use crate::DeviceWgpu;

pub struct VertexStateWgpu {
    vertex_buffer_state_info: Vec<VertexBufferStateInfo>,
}

impl VertexStateWgpu {
    pub fn new(_device: &DeviceWgpu, info: &VertexStateInfo) -> Self {
        Self {
            vertex_buffer_state_info: info.get_buffer_state_info_array().to_vec(),
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
