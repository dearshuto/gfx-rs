use sjgfx_interface::{
    AttributeFormat, CommandBufferInfo, DeviceInfo, ICommandBuffer, IDevice, IQueue, IShader,
    IVertexState, QueueInfo, ShaderInfo, VertexAttributeStateInfo, VertexBufferStateInfo,
    VertexStateInfo,
};
use sjgfx_wsys::{CommandBufferWsys, DeviceWsys, QueueWsys, ShaderWsys, VertexStateWsys};

fn main() {
    let mut instance = sjvi::web_sys::Instance::new();
    let id = instance.create_display();
    let display = instance.try_get_display(&id).unwrap();

    let mut device = DeviceWsys::new_with_surface(&DeviceInfo::new(), &display);
    let mut queue = QueueWsys::new(&mut device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWsys::new(&mut device, &CommandBufferInfo::new());

    let shader = ShaderWsys::new(
        &mut device,
        &ShaderInfo::new()
            .set_vertex_shader_source(include_str!("resources/hello_3d.vs"))
            .set_pixel_shader_source(include_str!("resources/hello_3d.fs")),
    );
    // 頂点ステート
    let vertex_attribute_state_info_array = [
        VertexAttributeStateInfo::new()
            .set_slot(0)
            .set_format(AttributeFormat::Float32_32_32)
            .set_offset(0)
            .set_buffer_index(0),
        VertexAttributeStateInfo::new()
            .set_slot(1)
            .set_format(AttributeFormat::Float32_32_32)
            .set_offset((std::mem::size_of::<f32>() * 3) as i64)
            .set_buffer_index(0),
    ];
    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride((std::mem::size_of::<f32>() * 6) as i64)];
    let vertex_state_info = VertexStateInfo::new()
        .set_attribute_state_info_array(vertex_attribute_state_info_array.into_iter())
        .set_buffer_state_info_array(vertex_buffer_state_info_array.into_iter());
    let vertex_state = VertexStateWsys::new(&device, &vertex_state_info);

    command_buffer.begin();
    command_buffer.set_shader(&shader);
    command_buffer.set_vertex_state(&vertex_state);
    command_buffer.end();

    queue.execute(&command_buffer);
}
