use std::sync::Arc;

use sjgfx_interface::{AttributeFormat, IVertexState, VertexBufferStateInfo, VertexStateInfo};
use wgpu::{VertexBufferLayout, VertexStepMode};

use crate::DeviceWgpu;

pub struct VertexStateWgpu {
    vertex_attribute: Arc<Vec<wgpu::VertexAttribute>>,
    verex_buffer_state_infos: Arc<Vec<VertexBufferStateInfo>>,
}

impl VertexStateWgpu {
    pub fn new(_device: &DeviceWgpu, info: &VertexStateInfo) -> Self {
        let vertex_attributes = info
            .get_attribute_state_info_array()
            .iter()
            .map(|x| wgpu::VertexAttribute {
                format: Self::convert_format_to_wgpu(x.get_format()),
                offset: x.get_offset() as u64,
                shader_location: x.get_slot() as u32,
            })
            .collect::<Vec<wgpu::VertexAttribute>>()
            .to_vec();
        let vertex_attributes = Arc::new(vertex_attributes);

        Self {
            vertex_attribute: vertex_attributes,
            verex_buffer_state_infos: Arc::new(info.get_buffer_state_info_array().to_vec()),
        }
    }

    pub fn view(&self) -> VertexStateView {
        VertexStateView::new(self)
    }

    fn clone_vertex_attributes(&self) -> Arc<Vec<wgpu::VertexAttribute>> {
        self.vertex_attribute.clone()
    }

    fn clone_vertex_buffer_state_infos(&self) -> Arc<Vec<VertexBufferStateInfo>> {
        self.verex_buffer_state_infos.clone()
    }

    fn convert_format_to_wgpu(format: &AttributeFormat) -> wgpu::VertexFormat {
        match format {
            AttributeFormat::Float32_32 => wgpu::VertexFormat::Float32x2,
            AttributeFormat::Float32_32_32 => wgpu::VertexFormat::Float32x3,
        }
    }
}

pub struct VertexStateView {
    vertex_attributes: Arc<Vec<wgpu::VertexAttribute>>,
    vertex_buffer_state_infos: Arc<Vec<VertexBufferStateInfo>>,
}

impl VertexStateView {
    pub fn new(vertex_state: &VertexStateWgpu) -> Self {
        Self {
            vertex_attributes: vertex_state.clone_vertex_attributes(),
            vertex_buffer_state_infos: vertex_state.clone_vertex_buffer_state_infos(),
        }
    }

    pub fn get_vertex_buffer_layout<'a>(&'a self) -> Vec<wgpu::VertexBufferLayout<'a>> {
        let vertex_buffer_layouts = self
            .vertex_buffer_state_infos
            .iter()
            .map(|x| {
                let step_mode = if x.get_divisor() == 0 {
                    VertexStepMode::Vertex
                } else {
                    VertexStepMode::Instance
                };
                let vertex_buffer_layout = VertexBufferLayout {
                    array_stride: x.get_stride() as u64,
                    step_mode,
                    attributes: &self.vertex_attributes,
                };
                vertex_buffer_layout
            })
            .collect();

        vertex_buffer_layouts
    }
}

impl IVertexState for VertexStateWgpu {
    type DeviceType = DeviceWgpu;

    fn new(device: &Self::DeviceType, info: &VertexStateInfo) -> Self {
        Self::new(device, info)
    }
}
