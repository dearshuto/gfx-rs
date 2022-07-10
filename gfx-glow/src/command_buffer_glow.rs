use sjgfx_interface::{
    CommandBufferInfo, ICommandBuffer, ScissorStateInfo, VertexAttributeStateInfo,
    VertexBufferStateInfo,
};

use crate::{
    BufferGlow, ColorTargetViewGlow, DepthStencilViewGlow, DeviceGlow, SamplerGlow, ShaderGlow,
    TextureGlow, TextureViewGlow, VertexStateGlow,
};

pub struct CommandBufferGlow {
    shader: Option<glow::Program>,
    render_target: [Option<glow::Texture>; 8],
    depth_buffer: Option<glow::Texture>,
    samplers: [Option<glow::Sampler>; 8],
    textures: [Option<glow::Texture>; 8],
    constant_buffers: [Option<glow::Buffer>; 8],
    unordered_access_buffer: [Option<glow::Buffer>; 8],
    vertex_buffers: [Option<glow::Buffer>; 8],
    vertex_attribute_state_infos: Option<Vec<VertexAttributeStateInfo>>,
    vertex_buffer_state_infos: Option<Vec<VertexBufferStateInfo>>,
    scissor_state: Option<ScissorStateInfo>,
    draw_command: Option<DrawCommand>,
}

impl CommandBufferGlow {
    pub fn set_shader(&mut self, shader: &ShaderGlow) {
        self.shader = Some(shader.get_program());
    }

    pub fn try_get_program(&self) -> Option<glow::Program> {
        self.shader
    }

    pub fn try_get_command(&self) -> Option<&DrawCommand> {
        self.draw_command.as_ref()
    }
}

impl ICommandBuffer for CommandBufferGlow {
    type DeviceType = DeviceGlow;
    type BufferType = BufferGlow;
    type ColorTargetViewType = ColorTargetViewGlow;
    type DepthStencilViewType = DepthStencilViewGlow;
    type SamplerType = SamplerGlow;
    type ShaderType = ShaderGlow;
    type TextureType = TextureGlow;
    type TextureViewType = TextureViewGlow;
    type VertexStateType = VertexStateGlow;

    fn new(_device: &Self::DeviceType, _info: &CommandBufferInfo) -> Self {
        Self {
            shader: None,
            render_target: Default::default(),
            depth_buffer: None,
            samplers: Default::default(),
            textures: Default::default(),
            constant_buffers: Default::default(),
            unordered_access_buffer: Default::default(),
            vertex_buffers: Default::default(),
            vertex_attribute_state_infos: None,
            vertex_buffer_state_infos: None,
            scissor_state: None,
            draw_command: None,
        }
    }

    fn begin(&mut self) {}

    fn end(&mut self) {}

    fn clear_color(
        &mut self,
        _color_target_view: &mut Self::ColorTargetViewType,
        _red: f32,
        _green: f32,
        _blue: f32,
        _alpha: f32,
        _texture_array_range: sjgfx_interface::TextureArrayRange,
    ) {
    }

    fn set_render_targets(
        &mut self,
        color_target_views: &[&Self::ColorTargetViewType],
        depth_stencil_view: Option<&Self::DepthStencilViewType>,
    ) {
        for index in 0..color_target_views.len() {
            self.render_target[index] = Some(color_target_views[index].get_texture());
        }

        if let Some(depth_stencil_view) = depth_stencil_view {
            self.depth_buffer = Some(depth_stencil_view.get_texture());
        }
    }

    fn set_shader(&mut self, shader: &Self::ShaderType) {
        self.shader = Some(shader.get_program());
    }

    fn set_sampler(&mut self, index: i32, sampler: &Self::SamplerType) {
        self.samplers[index as usize] = Some(sampler.get_handle());
    }

    fn set_texture(&mut self, index: i32, texture_view: &Self::TextureViewType) {
        self.textures[index as usize] = Some(texture_view.get_handle());
    }

    fn set_image(&mut self, _index: i32, _texture: &Self::TextureViewType) {
        todo!()
    }

    fn set_constant_buffer(&mut self, index: i32, buffer: &Self::BufferType) {
        self.constant_buffers[index as usize] = Some(buffer.get_handle());
    }

    fn set_unordered_access_buffer(&mut self, index: i32, buffer: &Self::BufferType) {
        self.unordered_access_buffer[index as usize] = Some(buffer.get_handle());
    }

    fn set_vertex_buffer(&mut self, index: i32, buffer: &Self::BufferType) {
        self.vertex_buffers[index as usize] = Some(buffer.get_handle());
    }

    fn set_vertex_state(&mut self, vertex_state: &Self::VertexStateType) {
        self.vertex_attribute_state_infos =
            Some(vertex_state.get_vertex_attribute_state_infos().to_vec());
        self.vertex_buffer_state_infos =
            Some(vertex_state.get_vertex_buffer_state_info_array().to_vec());
    }

    fn set_scissor(&mut self, scissor_state_info: &sjgfx_interface::ScissorStateInfo) {
        self.scissor_state = Some(scissor_state_info.clone());
    }

    fn dispatch(&mut self, count_x: i32, count_y: i32, count_z: i32) {
        let info = DispatchInfo {
            count_x: count_x as u32,
            count_y: count_y as u32,
            count_z: count_z as u32,
        };
        self.draw_command = Some(DrawCommand::Dispatch(info));
    }

    fn draw(
        &mut self,
        _primitive_topology: sjgfx_interface::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        let info = DrawInfo {
            primitive_topology: glow::TRIANGLES,
            vertex_count,
            vertex_offset,
        };
        self.draw_command = Some(DrawCommand::Draw(info));
    }

    fn draw_instanced(
        &mut self,
        _primitive_topology: sjgfx_interface::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        let info = DrawInstancedInfo {
            primitive_topology: glow::TRIANGLES,
            vertex_count,
            vertex_offset,
            instance_count,
            base_instance: base_instance as u32,
        };
        self.draw_command = Some(DrawCommand::DrawInstanced(info));
    }

    fn draw_indexed(
        &mut self,
        _primitive_topology: sjgfx_interface::PrimitiveTopology,
        _index_format: sjgfx_interface::IndexFormat,
        index_buffer: &Self::BufferType,
        index_count: i32,
        base_vertex: i32,
    ) {
        let info = DrawIndexedInfo {
            primitive_topology: glow::TRIANGLES,
            mode: glow::UNSIGNED_INT,
            index_count,
            base_vertex,
            buffer: index_buffer.get_handle(),
        };
        self.draw_command = Some(DrawCommand::DrawIndexed(info));
    }

    fn draw_indexed_instanced(
        &mut self,
        _primitive_topology: sjgfx_interface::PrimitiveTopology,
        _index_format: sjgfx_interface::IndexFormat,
        index_buffer: &Self::BufferType,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        let info = DrawIndexedInstancedInfo {
            primitive_topology: glow::TRIANGLES,
            mode: glow::UNSIGNED_INT,
            index_count,
            base_vertex,
            buffer: index_buffer.get_handle(),
            instance_count,
            base_instance: base_instance as u32,
        };
        self.draw_command = Some(DrawCommand::DrawIndexedInstanced(info));
    }
}

pub enum DrawCommand {
    Draw(DrawInfo),
    DrawInstanced(DrawInstancedInfo),
    DrawIndexed(DrawIndexedInfo),
    DrawIndexedInstanced(DrawIndexedInstancedInfo),
    Dispatch(DispatchInfo),
}

pub struct DrawInfo {
    pub primitive_topology: u32,
    pub vertex_count: i32,
    pub vertex_offset: i32,
}

pub struct DrawInstancedInfo {
    pub primitive_topology: u32,
    pub vertex_count: i32,
    pub vertex_offset: i32,
    pub instance_count: i32,
    pub base_instance: u32,
}

pub struct DrawIndexedInfo {
    pub primitive_topology: u32,
    pub mode: u32,
    pub index_count: i32,
    pub base_vertex: i32,
    pub buffer: glow::Buffer,
}

pub struct DrawIndexedInstancedInfo {
    pub primitive_topology: u32,
    pub mode: u32,
    pub index_count: i32,
    pub base_vertex: i32,
    pub buffer: glow::Buffer,
    pub instance_count: i32,
    pub base_instance: u32,
}

pub struct DispatchInfo {
    pub count_x: u32,
    pub count_y: u32,
    pub count_z: u32,
}
