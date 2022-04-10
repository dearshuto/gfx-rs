use crate::{
    shader_api::IShader, IBuffer, IColorTargetView, IDepthStencilView, IDevice, ISampler, ITexture,
    ITextureView, IndexFormat, PrimitiveTopology, TextureArrayRange,
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
    type SamplerType: ISampler;
    type ShaderType: IShader;
    type TextureType: ITexture;
    type TextureViewType: ITextureView;
    type VertexStateType;

    fn new(device: &Self::DeviceType, info: &CommandBufferInfo) -> Self;

    fn begin(&mut self);

    fn end(&mut self);

    fn clear_color(
        &mut self,
        color_target_view: &mut Self::ColorTargetViewType,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        texture_array_range: TextureArrayRange,
    );

    fn set_render_targets(
        &mut self,
        color_target_views: &[&Self::ColorTargetViewType],
        depth_stencil_view: Option<&Self::DepthStencilViewType>,
    );

    fn set_shader(&mut self, shader: &Self::ShaderType);

    fn set_sampler(&mut self, index: i32, sampler: &Self::SamplerType);

    fn set_texture(&mut self, index: i32, texture_view: &Self::TextureViewType);

    fn set_image(&mut self, index: i32, texture: &Self::TextureViewType);

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
