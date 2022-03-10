use crate::{PrimitiveTopology, IndexFormat, IBuffer, IDevice, IColorTargetView, IDepthStencilView, shader_api::IShader, ITexture};

pub struct CommandBufferInfo {}

impl CommandBufferInfo {
    pub fn new() -> Self {
        CommandBufferInfo {}
    }
}

pub trait ICommandBuffer<'a> {
    type DeviceType: IDevice;
    type BufferType: IBuffer<'a>;
    type ColorTargetViewType: IColorTargetView<'a>;
    type DepthStencilViewType: IDepthStencilView<'a>;
    type ShaderType: IShader<'a>;
    type TextureType: ITexture<'a>;
    type VertexStateType;

    fn new(device: &'a Self::DeviceType, info: &CommandBufferInfo) -> Self;

    fn begin(&mut self);

    fn enf(&mut self);

    fn set_render_targets<TIterator>(&mut self, color_target_views: TIterator, depth_stencil_view: Option<&'a Self::DepthStencilViewType>)
        where TIterator: Iterator<Item = Self::ColorTargetViewType>;

    fn set_shader(&mut self, shader: &'a Self::ShaderType);

    fn set_constant_buffer(&mut self, index: i32, buffer: &'a Self::BufferType);

    fn set_unordered_access_buffer(&mut self, index: i32, buffer: &'a Self::BufferType);

    fn set_vertex_buffer(&mut self, index: i32, buffer: &'a Self::BufferType);

    fn set_vertex_state(&mut self, vertex_state: &'a Self::VertexStateType);

    fn dispatch(&mut self, count_x: i32, count_y: i32, count_z: i32);

    fn draw(&mut self, primitive_topology:  PrimitiveTopology, vertex_count: i32, vertex_offset: i32);

    fn draw_indexed(&mut self, primitive_topology: PrimitiveTopology, index_format: IndexFormat, index_buffer: &'a Self::BufferType, index_count: i32, base_vertex: i32);
}
