use std::sync::Arc;

use sjgfx_interface::{
    ICommandBuffer, PrimitiveTopology, VertexAttributeStateInfo, VertexBufferStateInfo,
};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext as GL, WebGlVertexArrayObject};
use web_sys::{WebGlBuffer, WebGlProgram, WebGlSampler, WebGlTexture};

use crate::{
    BufferWsys, ColorTargetViewWsys, DepthStencilViewWsys, DeviceWsys, SamplerWsys, ShaderWsys,
    TextureViewWsys, TextureWsys, VertexStateWsys,
};

pub struct CommandBufferWsys {
    gl: Arc<WebGl2RenderingContext>,
    shader: Option<WebGlProgram>,

    // デスクリプタたち
    samplers: [Option<WebGlSampler>; 8],
    textures: [Option<WebGlTexture>; 8],
    constant_buffers: [Option<WebGlBuffer>; 8],

    vertex_buffers: [Option<WebGlBuffer>; 8],

    // 頂点ステート
    vertex_attribute_state_infos: Option<Vec<VertexAttributeStateInfo>>,
    vertex_buffer_state_infos: Option<Vec<VertexBufferStateInfo>>,

    command: Option<Command>,

    // VAO
    vertex_array_object: Option<WebGlVertexArrayObject>,
}

impl CommandBufferWsys {
    pub fn try_get_shader(&self) -> Option<&WebGlProgram> {
        self.shader.as_ref()
    }

    pub fn try_get_command(&self) -> Option<&Command> {
        self.command.as_ref()
    }

    pub fn try_get_vertex_array_object(&self) -> Option<WebGlVertexArrayObject> {
        if let Some(vao) = &self.vertex_array_object {
            Some(vao.clone())
        } else {
            None
        }
    }
}

impl ICommandBuffer for CommandBufferWsys {
    type DeviceType = DeviceWsys;
    type BufferType = BufferWsys;
    type ColorTargetViewType = ColorTargetViewWsys;
    type DepthStencilViewType = DepthStencilViewWsys;
    type SamplerType = SamplerWsys;
    type ShaderType = ShaderWsys;
    type TextureType = TextureWsys;
    type TextureViewType = TextureViewWsys;
    type VertexStateType = VertexStateWsys;

    fn new(device: &Self::DeviceType, _info: &sjgfx_interface::CommandBufferInfo) -> Self {
        Self {
            gl: device.clone_context(),
            shader: None,
            samplers: Default::default(),
            textures: Default::default(),
            constant_buffers: Default::default(),
            vertex_buffers: Default::default(),
            vertex_attribute_state_infos: None,
            vertex_buffer_state_infos: None,
            command: None,
            vertex_array_object: None,
        }
    }

    fn begin(&mut self) {}

    fn end(&mut self) {
        // VAO がなかったら作る
        if self.vertex_array_object.is_none() {
            let new_vao = self.gl.create_vertex_array().unwrap();
            self.vertex_array_object = Some(new_vao);
        }

        // VAO の更新を開始
        self.gl.bind_vertex_array(self.vertex_array_object.as_ref());

        // 頂点ステート/頂点バッファ
        if let Some(vertex_attribute_state_info_array) = &self.vertex_attribute_state_infos {
            for info in vertex_attribute_state_info_array {
                // 頂点バッファをバインド
                let buffer = &self.vertex_buffers[info.get_buffer_index() as usize];
                self.gl.bind_buffer(GL::ARRAY_BUFFER, buffer.as_ref());

                // 頂点アトリビュートを生成
                let slot = info.get_slot() as u32;
                self.gl.enable_vertex_attrib_array(slot);

                // 頂点アトリビュートの設定
                let vertex_buffer_info = &self.vertex_buffer_state_infos.as_ref().unwrap()
                    [info.get_buffer_index() as usize];

                match info.get_format() {
                    sjgfx_interface::AttributeFormat::Uint32 => {
                        self.gl.vertex_attrib_i_pointer_with_i32(
                            slot,
                            1,
                            GL::UNSIGNED_INT,
                            2 * std::mem::size_of::<u32>() as i32,
                            info.get_offset() as i32,
                        )
                    }
                    sjgfx_interface::AttributeFormat::Float32_32 => {
                        self.gl.vertex_attrib_pointer_with_i32(
                            slot,
                            2,
                            GL::FLOAT,
                            false,
                            vertex_buffer_info.get_stride() as i32,
                            info.get_offset() as i32,
                        )
                    }
                    sjgfx_interface::AttributeFormat::Float32_32_32 => {
                        self.gl.vertex_attrib_pointer_with_i32(
                            slot,
                            3,
                            GL::FLOAT,
                            false,
                            vertex_buffer_info.get_stride() as i32,
                            info.get_offset() as i32,
                        )
                    }
                }

                // 頂点バッファのバインドを解除
                self.gl.bind_buffer(GL::ARRAY_BUFFER, None);
            }
        }
    }

    fn clear_color(
        &mut self,
        _color_target_view: &mut Self::ColorTargetViewType,
        _red: f32,
        _green: f32,
        _blue: f32,
        _alpha: f32,
        _texture_array_range: sjgfx_interface::TextureArrayRange,
    ) {
        todo!()
    }

    fn set_render_targets(
        &mut self,
        _color_target_views: &[&Self::ColorTargetViewType],
        _depth_stencil_view: Option<&Self::DepthStencilViewType>,
    ) {
        todo!()
    }

    fn set_shader(&mut self, shader: &Self::ShaderType) {
        self.shader = Some(shader.clone_program());
    }

    fn set_sampler(&mut self, index: i32, sampler: &Self::SamplerType) {
        self.samplers[index as usize] = Some(sampler.clone_sampler());
    }

    fn set_texture(&mut self, index: i32, texture_view: &Self::TextureViewType) {
        self.textures[index as usize] = Some(texture_view.clone_texture());
    }

    fn set_image(&mut self, _index: i32, _texture: &Self::TextureViewType) {
        todo!()
    }

    fn set_constant_buffer(&mut self, index: i32, buffer: &Self::BufferType) {
        self.constant_buffers[index as usize] = Some(buffer.clone_buffer());
    }

    fn set_unordered_access_buffer(&mut self, _index: i32, _buffer: &Self::BufferType) {
        todo!()
    }

    fn set_vertex_buffer(&mut self, index: i32, buffer: &Self::BufferType) {
        self.vertex_buffers[index as usize] = Some(buffer.clone_buffer());
    }

    fn set_vertex_state(&mut self, vertex_state: &Self::VertexStateType) {
        self.vertex_attribute_state_infos = Some(vertex_state.get_attribute_state_infos().to_vec());
        self.vertex_buffer_state_infos = Some(vertex_state.get_buffer_state_infos().to_vec());
    }

    fn set_scissor(&mut self, _scissor_state_info: &sjgfx_interface::ScissorStateInfo) {
        todo!()
    }

    fn dispatch(&mut self, count_x: i32, count_y: i32, count_z: i32) {
        let dispatch_info = DispatchInfo {
            count_x,
            count_y,
            count_z,
        };
        let command = Command::Dispatch(dispatch_info);
        self.command = Some(command);
    }

    fn draw(
        &mut self,
        _primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        let info = DrawInfo {
            primitive_topology: GL::TRIANGLES,
            vertex_count,
            offset: vertex_offset,
        };
        let command = Command::Draw(info);
        self.command = Some(command);
    }

    fn draw_instanced(
        &mut self,
        _primitive_topology: PrimitiveTopology,
        _vertex_count: i32,
        _vertex_offset: i32,
        _instance_count: i32,
        _base_instance: i32,
    ) {
        todo!()
    }

    fn draw_indexed(
        &mut self,
        _primitive_topology: PrimitiveTopology,
        _index_format: sjgfx_interface::IndexFormat,
        _index_buffer: &Self::BufferType,
        _index_count: i32,
        _base_vertex: i32,
    ) {
        todo!()
    }

    fn draw_indexed_instanced(
        &mut self,
        _primitive_topology: PrimitiveTopology,
        _index_format: sjgfx_interface::IndexFormat,
        _index_buffer: &Self::BufferType,
        _index_count: i32,
        _base_vertex: i32,
        _instance_count: i32,
        _base_instance: i32,
    ) {
        todo!()
    }
}

pub enum Command {
    Draw(DrawInfo),
    Dispatch(DispatchInfo),
}

pub struct DrawInfo {
    pub primitive_topology: u32,
    pub vertex_count: i32,
    pub offset: i32,
}

pub struct DispatchInfo {
    pub count_x: i32,
    pub count_y: i32,
    pub count_z: i32,
}