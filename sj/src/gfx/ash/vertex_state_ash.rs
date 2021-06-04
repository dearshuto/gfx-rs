use super::super::vertex_state_api::{IVertexState, VertexStateInfo};
use super::super::Device;

pub struct VertexStateImpl {}

impl IVertexState for VertexStateImpl {
    fn new(_device: &Device, _info: &VertexStateInfo) -> Self {
        Self {}
    }
}
