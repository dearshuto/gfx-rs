use super::super::vertex_state_api::{IVertexState, VertexStateInfo};
use super::super::Device;

pub struct VertexStateWgpu {}

impl IVertexState for VertexStateWgpu {
    fn new(_device: &Device, _info: &VertexStateInfo) -> Self {
        todo!();
    }
}
