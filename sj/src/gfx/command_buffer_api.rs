use super::{
    texture_api::{TextureArrayRange, TextureSubresource, TextureSubresourceRange},
    Buffer, BufferTextureCopyRegion, ColorTargetView, DepthStencilClearMode, DepthStencilView,
    Device, GpuAccess, GpuAddress, IndexFormat, Pipeline, PipelineStageBit, PrimitiveTopology,
    ShaderStage, Texture, TextureCopyRegion, TextureState, ViewportScissorState,
};
use crate::gfx::ClearColorValue;
use std::marker::PhantomData;

pub struct CommandBufferInfo {}

impl CommandBufferInfo {
    pub fn new() -> Self {
        CommandBufferInfo {}
    }
}

pub trait ICommandBufferImpl<'a> {
    fn new(device: &'a Device, info: &CommandBufferInfo) -> Self;

    fn begin(&mut self);

    fn end(&mut self);

    fn reset(&mut self);

    fn set_viewport_scissor_state(&mut self, viewport_scissor_state: &'a ViewportScissorState);

    fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>);

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: &GpuAddress,
        size: usize,
    );

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: &GpuAddress,
        size: u64,
    );

    fn clear_color(
        &mut self,
        color_target_view: &mut ColorTargetView,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        texture_array_range: Option<&TextureArrayRange>,
    );

    //fn clear_color_target(&mut self, clear_color: &ClearColorValue);

    fn clear_depth_stencil(
        &mut self,
        depth_stencil: &mut DepthStencilView,
        depth: f32,
        stencil: i32,
        clear_mode: &DepthStencilClearMode,
        texture_array_range: Option<&TextureArrayRange>,
    );

    fn set_render_targets(
        &mut self,
        color_target_views: &[&ColorTargetView],
        depth_stencil_state_view: Option<&DepthStencilView>,
    );

    fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &GpuAddress);

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
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
    );

    fn draw_indexed_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    );

    fn draw_indirect(&mut self, gpu_address: &GpuAddress);

    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32);

    fn set_texture_state_transition(
        &mut self,
        texture: &Texture,
        range: &TextureSubresourceRange,
        old_state: TextureState,
        old_stage_bit: PipelineStageBit,
        new_state: TextureState,
        new_stage_bit: PipelineStageBit,
    );

    fn copy_image(
        &mut self,
        dst_texture: &mut Texture,
        dst_subresource: &TextureSubresource,
        dst_offset_u: i32,
        dst_offset_v: i32,
        dst_offset_w: i32,
        src_texture: &Texture,
        src_copy_range: TextureCopyRegion,
    );

    fn copy_image_to_buffer(
        &mut self,
        dst_buffer: &mut Buffer,
        src_texture: &Texture,
        copy_region: &BufferTextureCopyRegion,
    );

    fn flush_memory(&mut self, gpu_access_flags: GpuAccess);
}

pub struct TCommandBufferInterface<'a, T: 'a>
where
    T: ICommandBufferImpl<'a>,
{
    command_buffer_impl: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: ICommandBufferImpl<'a>> TCommandBufferInterface<'a, T> {
    pub fn new(device: &'a Device, info: &CommandBufferInfo) -> Self {
        Self {
            command_buffer_impl: T::new(device, info),
            _marker: PhantomData,
        }
    }

    pub fn begin(&mut self) {
        self.command_buffer_impl.begin();
    }

    pub fn end(&mut self) {
        self.command_buffer_impl.end();
    }

    pub fn set_viewport_scissor_state(&mut self, viewport_scissor_state: &'a ViewportScissorState) {
        self.command_buffer_impl
            .set_viewport_scissor_state(viewport_scissor_state);
    }

    pub fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>) {
        self.command_buffer_impl.set_pipeline(pipeline);
    }

    pub fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: &GpuAddress,
        size: usize,
    ) {
        self.command_buffer_impl
            .set_constant_buffer(slot, stage, gpu_address, size);
    }

    pub fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: &GpuAddress,
        size: u64,
    ) {
        self.command_buffer_impl
            .set_unordered_access_buffer(slot, stage, gpu_address, size);
    }

    pub fn clear_color(
        &mut self,
        color_target_view: &mut ColorTargetView,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        texture_array_range: Option<&TextureArrayRange>,
    ) {
        self.command_buffer_impl.clear_color(
            color_target_view,
            red,
            green,
            blue,
            alpha,
            texture_array_range,
        );
    }

    pub fn clear_color_target(&mut self, _clear_color: &ClearColorValue) {}

    pub fn clear_depth_stencil(
        &mut self,
        depth_stencil: &mut DepthStencilView,
        depth: f32,
        stencil: i32,
        clear_mode: &DepthStencilClearMode,
        texture_array_range: Option<&TextureArrayRange>,
    ) {
        self.command_buffer_impl.clear_depth_stencil(
            depth_stencil,
            depth,
            stencil,
            clear_mode,
            texture_array_range,
        );
    }

    pub fn set_render_targets(
        &mut self,
        color_target_views: &[&ColorTargetView],
        depth_stencil_state_view: Option<&DepthStencilView>,
    ) {
        self.command_buffer_impl
            .set_render_targets(color_target_views, depth_stencil_state_view);
    }

    pub fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &GpuAddress) {
        self.command_buffer_impl
            .set_vertex_buffer(buffer_index, gpu_address);
    }

    pub fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        self.command_buffer_impl
            .draw(primitive_topology, vertex_count, vertex_offset);
    }

    pub fn draw_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        self.command_buffer_impl.draw_instanced(
            primitive_topology,
            vertex_count,
            vertex_offset,
            instance_count,
            base_instance,
        );
    }

    pub fn draw_indexed(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
    ) {
        self.command_buffer_impl.draw_indexed(
            primitive_topology,
            index_format,
            gpu_address,
            index_count,
            base_vertex,
        );
    }

    pub fn draw_indexed_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        self.command_buffer_impl.draw_indexed_instanced(
            primitive_topology,
            index_format,
            gpu_address,
            index_count,
            base_vertex,
            instance_count,
            base_instance,
        );
    }

    pub fn draw_indirect(&mut self, gpu_address: &GpuAddress) {
        self.command_buffer_impl.draw_indirect(gpu_address);
    }

    pub fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        self.command_buffer_impl
            .dispatch(group_count_x, group_count_y, group_count_z);
    }

    pub fn set_texture_state_transition(
        &mut self,
        texture: &Texture,
        range: &TextureSubresourceRange,
        old_state: TextureState,
        old_stage_bit: PipelineStageBit,
        new_state: TextureState,
        new_stage_bit: PipelineStageBit,
    ) {
        self.command_buffer_impl.set_texture_state_transition(
            texture,
            range,
            old_state,
            old_stage_bit,
            new_state,
            new_stage_bit,
        );
    }

    pub fn copy_image(
        &mut self,
        dst_texture: &mut Texture,
        dst_subresource: &TextureSubresource,
        dst_offset_u: i32,
        dst_offset_v: i32,
        dst_offset_w: i32,
        src_texture: &Texture,
        src_copy_range: TextureCopyRegion,
    ) {
        self.command_buffer_impl.copy_image(
            dst_texture,
            dst_subresource,
            dst_offset_u,
            dst_offset_v,
            dst_offset_w,
            src_texture,
            src_copy_range,
        );
    }

    pub fn copy_image_to_buffer(
        &mut self,
        dst_buffer: &mut Buffer,
        src_texture: &Texture,
        copy_region: &BufferTextureCopyRegion,
    ) {
        self.command_buffer_impl
            .copy_image_to_buffer(dst_buffer, src_texture, copy_region);
    }

    pub fn flush_memory(&mut self, gpu_access_flags: GpuAccess) {
        self.command_buffer_impl.flush_memory(gpu_access_flags);
    }

    pub fn to_data(&'a self) -> &'a T {
        &self.command_buffer_impl
    }

    pub fn to_data_mut(&'a mut self) -> &'a mut T {
        &mut self.command_buffer_impl
    }
}
