use crate::{
    shader_api::IShader, IBuffer, IColorTargetView, IDepthStencilView, IDevice, ITexture,
    IndexFormat, PrimitiveTopology,
};

pub struct CommandBufferInfo {}

impl CommandBufferInfo {
    pub fn new() -> Self {
        CommandBufferInfo {}
    }
}

pub trait ICommandBuffer {
    type DeviceType: IDevice;
    type BufferType: IBuffer;
    type ColorTargetViewType: IColorTargetView;
    type DepthStencilViewType: IDepthStencilView;
    type ShaderType: IShader;
    type TextureType: ITexture;
    type VertexStateType;

    fn new(device: &Self::DeviceType, info: &CommandBufferInfo) -> Self;

    fn begin(&mut self);

    fn end(&mut self);

    fn set_render_targets<TIterator>(
        &mut self,
        color_target_views: TIterator,
        depth_stencil_view: Option<&Self::DepthStencilViewType>,
    ) where
        TIterator: Iterator<Item = Self::ColorTargetViewType>;

    fn set_shader(&mut self, shader: &Self::ShaderType);

    fn set_image(&mut self, index: i32, texture: &Self::TextureType);

    fn set_constant_buffer(&mut self, index: i32, buffer: &Self::BufferType);

    fn set_unordered_access_buffer(&mut self, index: i32, buffer: &Self::BufferType);

    fn set_vertex_buffer(&mut self, index: i32, buffer: &Self::BufferType);

    fn set_vertex_state(&mut self, vertex_state: &Self::VertexStateType);

    fn dispatch(&mut self, count_x: i32, count_y: i32, count_z: i32);

    fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    );

    fn draw_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    );

    fn draw_indexed(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        index_buffer: &Self::BufferType,
        index_count: i32,
        base_vertex: i32,
    );

    fn draw_indexed_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        index_buffer: &Self::BufferType,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    );
}
