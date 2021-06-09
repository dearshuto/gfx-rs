use super::super::vertex_state_api::{IVertexState, VertexStateInfo};
use super::super::Device;

pub struct VertexStateWgpu {}

impl IVertexState for VertexStateWgpu {
    fn new(device: &Device, info: &VertexStateInfo) -> Self {
        todo!();
    }
}
