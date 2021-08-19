use crate::gfx::{
    AttributeFormat, Device, DeviceInfo, VertexAttributeStateInfo, VertexBufferStateInfo,
    VertexState, VertexStateInfo,
};

#[test]
fn initialize() {
    let device = Device::new(&DeviceInfo::new());
    let vertex_attribute_state_info_array = [VertexAttributeStateInfo::new()
        .set_slot(0)
        .set_format(AttributeFormat::Float32_32)];
    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<f32>() as i64)];

    let vertex_state_info = VertexStateInfo::new()
        .set_attribute_state_info_array(&vertex_attribute_state_info_array)
        .set_buffer_state_info_array(&vertex_buffer_state_info_array);
    let _vertex_state = VertexState::new(&device, &vertex_state_info);
}
