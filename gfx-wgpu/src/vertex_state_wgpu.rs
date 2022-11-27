use std::{collections::HashMap, sync::Arc};

use sjgfx_interface::{AttributeFormat, IVertexState, VertexBufferStateInfo, VertexStateInfo};
use uuid::Uuid;
use wgpu::{VertexAttribute, VertexBufferLayout, VertexStepMode};

use crate::DeviceWgpu;

pub struct VertexStateWgpu {
    vertex_attribute_map: Arc<HashMap<i32, Vec<wgpu::VertexAttribute>>>,
    verex_buffer_state_infos: Arc<Vec<VertexBufferStateInfo>>,
    pub id: Uuid,
}

impl VertexStateWgpu {
    pub fn new(_device: &DeviceWgpu, info: &VertexStateInfo) -> Self {
        let mut map = HashMap::<i32, Vec<VertexAttribute>>::new();

        for attribute_info in info.get_attribute_state_info_array() {
            let buffer_index = attribute_info.get_buffer_index();
            if !map.contains_key(&buffer_index) {
                map.insert(buffer_index, Vec::new());
            }

            map.get_mut(&buffer_index)
                .unwrap()
                .push(wgpu::VertexAttribute {
                    format: Self::convert_format_to_wgpu(attribute_info.get_format()),
                    offset: attribute_info.get_offset() as u64,
                    shader_location: attribute_info.get_slot() as u32,
                });
        }

        Self {
            vertex_attribute_map: Arc::new(map),
            verex_buffer_state_infos: Arc::new(info.get_buffer_state_info_array().to_vec()),
            id: Uuid::new_v4(),
        }
    }

    pub fn view(&self) -> VertexStateView {
        VertexStateView::new(self)
    }

    fn clone_vertex_attributes(&self) -> Arc<HashMap<i32, Vec<wgpu::VertexAttribute>>> {
        self.vertex_attribute_map.clone()
    }

    fn clone_vertex_buffer_state_infos(&self) -> Arc<Vec<VertexBufferStateInfo>> {
        self.verex_buffer_state_infos.clone()
    }

    fn convert_format_to_wgpu(format: &AttributeFormat) -> wgpu::VertexFormat {
        match format {
            AttributeFormat::Uint32 => wgpu::VertexFormat::Uint32,
            AttributeFormat::Float32_32 => wgpu::VertexFormat::Float32x2,
            AttributeFormat::Float32_32_32 => wgpu::VertexFormat::Float32x3,
            AttributeFormat::Float32_32_32_32 => wgpu::VertexFormat::Float32x4,
        }
    }
}

pub struct VertexStateView {
    vertex_attributes: Arc<HashMap<i32, Vec<wgpu::VertexAttribute>>>,
    vertex_buffer_state_infos: Arc<Vec<VertexBufferStateInfo>>,
    pub id: Uuid,
}

impl VertexStateView {
    pub fn new(vertex_state: &VertexStateWgpu) -> Self {
        Self {
            vertex_attributes: vertex_state.clone_vertex_attributes(),
            vertex_buffer_state_infos: vertex_state.clone_vertex_buffer_state_infos(),
            id: vertex_state.id,
        }
    }

    pub fn get_vertex_buffer_layout<'a>(&'a self) -> Vec<wgpu::VertexBufferLayout<'a>> {
        let vertex_buffer_layouts = self
            .vertex_buffer_state_infos
            .iter()
            .enumerate()
            .map(|(index, x)| {
                let step_mode = if x.get_divisor() == 0 {
                    VertexStepMode::Vertex
                } else {
                    VertexStepMode::Instance
                };

                let vertex_attributes = self.vertex_attributes.get(&(index as i32)).unwrap();
                let vertex_buffer_layout = VertexBufferLayout {
                    array_stride: x.get_stride() as u64,
                    step_mode,
                    attributes: &vertex_attributes,
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
