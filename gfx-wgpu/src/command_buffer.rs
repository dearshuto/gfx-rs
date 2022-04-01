use std::sync::Arc;

use sjgfx_interface::{
    BufferTextureCopyRegion, CommandBufferInfo, ICommandBuffer, IndexFormat, PrimitiveTopology,
    ScissorStateInfo, ViewportStateInfo,
};
use wgpu::Extent3d;

use crate::{
    shader_wgpu::ShaderView, vertex_state_wgpu::VertexStateView, BufferWgpu, ColorTargetViewWgpu,
    DepthStencilViewWgpu, DeviceWgpu, SamplerWgpu, ShaderWgpu, TextureViewWgpu, TextureWgpu,
    VertexStateWgpu,
};

struct DrawInfo {
    #[allow(dead_code)]
    pub primitive_topology: PrimitiveTopology,
    pub vertex_count: u32,
    pub instance_count: u32,
    pub base_instance: u32,
}

struct DrawIndexedInfo {
    #[allow(dead_code)]
    pub primitive_topology: PrimitiveTopology,
    pub index_format: wgpu::IndexFormat,
    pub index_buffer: Arc<wgpu::Buffer>,
    pub index_count: u32,
    pub instance_count: u32,
    pub base_instance: u32,
}

enum DrawCommand {
    Draw(DrawInfo),
    DrawIndexed(DrawIndexedInfo),
}

pub struct CommandBufferWgpu {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,

    // レンダーターゲット
    color_target_view: Option<ColorTargetViewWgpu>,
    depth_stencil_view: Option<Arc<wgpu::TextureView>>,

    // ビューポートシザー
    viewport_state_info: Option<ViewportStateInfo>,
    scissor_state_info: Option<ScissorStateInfo>,

    shader: Option<ShaderView>,
    constant_buffers: [Option<Arc<wgpu::Buffer>>; 8],
    unordered_access_buffer: [Option<Arc<wgpu::Buffer>>; 8],
    dispatch_count: Option<(u32, u32, u32)>,

    // テクスチャ
    textures: [Option<Arc<wgpu::TextureView>>; 8],
    samplers: [Option<Arc<wgpu::Sampler>>; 8],
    images: [Option<Arc<wgpu::TextureView>>; 8],

    // Draw
    vertex_buffer: [Option<Arc<wgpu::Buffer>>; 8],
    vertex_state: Option<VertexStateView>,
    draw_command: Option<DrawCommand>,
}

impl CommandBufferWgpu {
    pub fn new(device: &DeviceWgpu, _info: &CommandBufferInfo) -> Self {
        Self {
            device: device.close_device(),
            queue: device.clone_queue(),

            color_target_view: None,
            depth_stencil_view: None,

            // ビューポートシザー
            viewport_state_info: None,
            scissor_state_info: None,

            shader: None,
            constant_buffers: [None, None, None, None, None, None, None, None],
            unordered_access_buffer: [None, None, None, None, None, None, None, None],

            // テクスチャ
            textures: [None, None, None, None, None, None, None, None],
            samplers: [None, None, None, None, None, None, None, None],
            images: Default::default(),

            dispatch_count: None,
            vertex_buffer: [None, None, None, None, None, None, None, None],
            vertex_state: None,
            draw_command: None,
        }
    }

    pub fn begin(&self) {}

    pub fn end(&self) {}

    pub fn set_render_targets<TIterator>(
        &mut self,
        mut color_target_views: TIterator,
        depth_stencil_view: Option<&DepthStencilViewWgpu>,
    ) where
        TIterator: Iterator<Item = ColorTargetViewWgpu>,
    {
        if let Some(color_target_view) = color_target_views.next() {
            self.color_target_view = Some(color_target_view);
        }

        if let Some(depth_stencil_view) = depth_stencil_view {
            self.depth_stencil_view = Some(depth_stencil_view.close_texture_view());
        } else {
            self.depth_stencil_view = None;
        }
    }

    pub fn set_viewport(&mut self, viewport_state_info: &ViewportStateInfo) {
        self.viewport_state_info = Some(viewport_state_info.clone());
    }

    pub fn set_scissor(&mut self, scissor_state_info: &ScissorStateInfo) {
        self.scissor_state_info = Some(scissor_state_info.clone());
    }

    pub fn set_shader(&mut self, shader: &ShaderWgpu) {
        self.shader = Some(shader.view());
    }

    pub fn set_constant_buffer(&mut self, index: i32, buffer: &BufferWgpu) {
        self.constant_buffers[index as usize] = Some(buffer.close_buffer());
    }

    pub fn set_unordered_access_buffer(&mut self, index: i32, buffer: &BufferWgpu) {
        self.unordered_access_buffer[index as usize] = Some(buffer.close_buffer());
    }

    pub fn set_texture_direct(&mut self, index: i32, texture: &TextureWgpu) {
        self.textures[index as usize] = Some(Arc::new(
            texture
                .get_texture()
                .create_view(&wgpu::TextureViewDescriptor::default()),
        ));
    }

    pub fn set_image(&mut self, index: i32, texture: &TextureViewWgpu) {
        let index = index as usize;
        self.images[index] = Some(texture.clone_texture_view());
    }

    pub fn set_sampler(&mut self, index: i32, sampler: &SamplerWgpu) {
        self.samplers[index as usize] = Some(sampler.clone_sampler());
    }

    pub fn set_texture(&mut self, index: i32, texture: &TextureViewWgpu) {
        self.textures[index as usize] = Some(texture.clone_texture_view());
    }

    pub fn set_vertex_buffer(&mut self, index: i32, buffer: &BufferWgpu) {
        self.vertex_buffer[index as usize] = Some(buffer.close_buffer());
    }

    pub fn set_vertex_state(&mut self, vertex_state: &VertexStateWgpu) {
        self.vertex_state = Some(vertex_state.view());
    }

    pub fn dispatch(
        &mut self,
        dispatch_count_x: i32,
        dispatch_count_y: i32,
        dispatch_count_z: i32,
    ) {
        self.dispatch_count = Some((
            dispatch_count_x as u32,
            dispatch_count_y as u32,
            dispatch_count_z as u32,
        ));
    }

    pub fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        self.draw_instanced(
            primitive_topology,
            vertex_count,
            vertex_offset,
            1, /*instance_count*/
            0, /*base_instnce*/
        );
    }

    pub fn draw_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        _vertex_offset: i32,
        instance_count: i32,
        base_instnce: i32,
    ) {
        let draw_info = DrawInfo {
            primitive_topology,
            vertex_count: vertex_count as u32,
            base_instance: base_instnce as u32,
            instance_count: instance_count as u32,
        };
        self.draw_command = Some(DrawCommand::Draw(draw_info));
    }

    pub fn draw_indexed(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        index_buffer: &BufferWgpu,
        index_count: i32,
        base_vertex: i32,
    ) {
        self.draw_indexed_instanced(
            primitive_topology,
            index_format,
            index_buffer,
            index_count,
            base_vertex,
            1, /*instance_count*/
            0, /*base_instance*/
        );
    }

    pub fn draw_indexed_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        index_buffer: &BufferWgpu,
        index_count: i32,
        _base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        let index_format_wgpu = match index_format {
            IndexFormat::Uint32 => wgpu::IndexFormat::Uint32,
        };

        let draw_indexed_info = DrawIndexedInfo {
            primitive_topology,
            index_format: index_format_wgpu,
            index_buffer: index_buffer.close_buffer(),
            index_count: index_count as u32,
            instance_count: instance_count as u32,
            base_instance: base_instance as u32,
        };
        self.draw_command = Some(DrawCommand::DrawIndexed(draw_indexed_info));
    }

    pub fn copy_image_to_buffer(
        &mut self,
        buffer: &BufferWgpu,
        texture: &TextureWgpu,
        copy_region: BufferTextureCopyRegion,
    ) {
        let mut command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let copy_size = Extent3d {
            width: copy_region.get_image_width() as u32,
            height: copy_region.get_image_height() as u32,
            depth_or_array_layers: 0,
        };
        let image = texture.close_texture();
        let image_copy = image.as_image_copy();
        let image_copy_buffer = wgpu::ImageCopyBuffer {
            buffer: buffer.get_buffer(),
            layout: wgpu::ImageDataLayout {
                offset: copy_region.get_offset() as u64,
                bytes_per_row: None,
                rows_per_image: None,
            },
        };
        command_encoder.copy_texture_to_buffer(image_copy, image_copy_buffer, copy_size);
        self.queue.submit(Some(command_encoder.finish()));
    }

    pub(crate) fn build_command(&self) -> Option<wgpu::CommandBuffer> {
        if let Some(shader) = &self.shader {
            if shader.is_compute() {
                return Some(self.build_compute_command());
            } else {
                return Some(self.build_graphics_command());
            }
        } else {
            return None;
        }
    }

    fn build_compute_command(&self) -> wgpu::CommandBuffer {
        let bind_group = self.create_bind_group();
        let mut command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut compute_pass =
                command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });

            let compute_pipeline = self.shader.as_ref().unwrap().get_compute_pipeline();
            compute_pass.set_pipeline(&compute_pipeline);

            compute_pass.set_bind_group(0, &bind_group, &[]);

            let (dispatch_count_x, dispatch_cout_y, dispatch_count_z) =
                *self.dispatch_count.as_ref().unwrap();
            compute_pass.dispatch(dispatch_count_x, dispatch_cout_y, dispatch_count_z);
        }

        command_encoder.finish()
    }

    fn build_graphics_command(&self) -> wgpu::CommandBuffer {
        // レンダーターゲット
        let color_target_view = self.color_target_view.as_ref().unwrap();

        let vertex_shader_module = self.shader.as_ref().unwrap().get_vertex_shader_module();
        let pixel_shader_module = self.shader.as_ref().unwrap().get_pixel_shader_module();

        // 頂点ステート
        let vertex_buffer_layout = if let Some(vertex_state) = &self.vertex_state {
            vertex_state.get_vertex_buffer_layout()
        } else {
            vec![]
        };

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: None,
                vertex: wgpu::VertexState {
                    module: &vertex_shader_module,
                    entry_point: "main",
                    buffers: &vertex_buffer_layout,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &pixel_shader_module,
                    entry_point: "main",
                    targets: &[color_target_view.get_texture_format().into()],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: self.create_depth_stencil_state(),
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });
        let bind_group = self.create_bind_group();
        let mut command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: color_target_view.get_texture_view(),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                        store: true,
                    },
                }],
                depth_stencil_attachment: self.create_render_pass_depth_stencil_attachment(),
            });

            // パイプライン
            render_pass.set_pipeline(&render_pipeline);

            // デスクリプタたち
            render_pass.set_bind_group(0, &bind_group, &[]);

            // ビューポート
            if let Some(viewport_state_info) = &self.viewport_state_info {
                render_pass.set_viewport(
                    viewport_state_info.get_origin_x(),
                    viewport_state_info.get_origin_y(),
                    viewport_state_info.get_width(),
                    viewport_state_info.get_height(),
                    -1.0, /*min_depth*/
                    1.0,  /*max_depth*/
                )
            }

            // シザリング
            if let Some(scissor_state_info) = &self.scissor_state_info {
                render_pass.set_scissor_rect(
                    scissor_state_info.get_origin_x() as u32,
                    scissor_state_info.get_origin_y() as u32,
                    scissor_state_info.get_width() as u32,
                    scissor_state_info.get_height() as u32,
                );
            }

            // 頂点バッファ
            if let Some(vertex_buffer) = &self.vertex_buffer[0] {
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            }

            // 描画
            if let Some(draw_command) = &self.draw_command {
                match draw_command {
                    DrawCommand::Draw(ref draw_info) => {
                        render_pass.draw(
                            0..draw_info.vertex_count,
                            draw_info.base_instance..draw_info.instance_count,
                        );
                    }
                    DrawCommand::DrawIndexed(ref draw_indexed_info) => {
                        let buffer_slice = draw_indexed_info.index_buffer.slice(..);
                        render_pass.set_index_buffer(buffer_slice, draw_indexed_info.index_format);
                        render_pass.draw_indexed(
                            0..draw_indexed_info.index_count,
                            0,
                            draw_indexed_info.base_instance..draw_indexed_info.instance_count,
                        );
                    }
                }
            }
        }
        command_encoder.finish()
    }

    fn create_bind_group(&self) -> wgpu::BindGroup {
        let mut entries = Vec::new();

        // バッファ
        for index in 0..self.unordered_access_buffer.len() {
            if let Some(unordered_access_buffer) = &self.unordered_access_buffer[index] {
                entries.push(wgpu::BindGroupEntry {
                    binding: index as u32,
                    resource: unordered_access_buffer.as_entire_binding(),
                });
            }
        }

        // 定数バッファ
        for index in 0..self.constant_buffers.len() {
            if let Some(constant_buffer) = &self.constant_buffers[index] {
                entries.push(wgpu::BindGroupEntry {
                    binding: index as u32,
                    resource: constant_buffer.as_entire_binding(),
                });
            }
        }

        // テクスチャ
        for index in 0..self.textures.len() {
            if let Some(texture) = &self.textures[index] {
                entries.push(wgpu::BindGroupEntry {
                    binding: index as u32,
                    resource: wgpu::BindingResource::TextureView(texture),
                })
            }
        }

        // サンプラ
        for index in 0..self.samplers.len() {
            if let Some(sampler) = &self.samplers[index] {
                entries.push(wgpu::BindGroupEntry {
                    binding: index as u32,
                    resource: wgpu::BindingResource::Sampler(sampler),
                })
            }
        }

        // イメージ
        for index in 0..self.images.len() {
            if let Some(image) = &self.images[index] {
                entries.push(wgpu::BindGroupEntry {
                    binding: index as u32,
                    resource: wgpu::BindingResource::TextureView(image),
                })
            }
        }

        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: self.shader.as_ref().unwrap().get_bind_group_layout(),
            entries: &entries,
        })
    }

    fn create_depth_stencil_state(&self) -> Option<wgpu::DepthStencilState> {
        if let Some(_depth_stencil_view) = &self.depth_stencil_view {
            Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState {
                    constant: 2,
                    slope_scale: 2.0,
                    clamp: 0.0,
                },
            })
        } else {
            None
        }
    }

    fn create_render_pass_depth_stencil_attachment<'f>(
        &'f self,
    ) -> Option<wgpu::RenderPassDepthStencilAttachment<'f>> {
        if let Some(depth_stencil_view) = &self.depth_stencil_view {
            Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_stencil_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            })
        } else {
            None
        }
    }
}

impl ICommandBuffer for CommandBufferWgpu {
    type DeviceType = DeviceWgpu;
    type BufferType = BufferWgpu;
    type ColorTargetViewType = ColorTargetViewWgpu;
    type DepthStencilViewType = DepthStencilViewWgpu;
    type SamplerType = SamplerWgpu;
    type ShaderType = ShaderWgpu;
    type TextureType = TextureWgpu;
    type TextureViewType = TextureViewWgpu;
    type VertexStateType = VertexStateWgpu;

    fn new(device: &Self::DeviceType, info: &CommandBufferInfo) -> Self {
        Self::new(device, info)
    }

    fn begin(&mut self) {
        CommandBufferWgpu::begin(&self);
    }

    fn end(&mut self) {
        CommandBufferWgpu::end(&self);
    }

    fn set_render_targets<TIterator>(
        &mut self,
        color_target_views: TIterator,
        depth_stencil_view: Option<&Self::DepthStencilViewType>,
    ) where
        TIterator: Iterator<Item = Self::ColorTargetViewType>,
    {
        self.set_render_targets(color_target_views, depth_stencil_view)
    }

    fn set_shader(&mut self, shader: &Self::ShaderType) {
        self.set_shader(shader);
    }

    fn set_sampler(&mut self, index: i32, sampler: &Self::SamplerType) {
        self.set_sampler(index, sampler);
    }

    fn set_texture(&mut self, index: i32, texture_view: &Self::TextureViewType) {
        self.set_texture(index, texture_view);
    }

    fn set_image(&mut self, index: i32, texture: &Self::TextureViewType) {
        self.set_image(index, texture);
    }

    fn set_constant_buffer(&mut self, index: i32, buffer: &Self::BufferType) {
        self.set_constant_buffer(index, buffer);
    }

    fn set_unordered_access_buffer(&mut self, index: i32, buffer: &Self::BufferType) {
        self.set_unordered_access_buffer(index, buffer);
    }

    fn set_vertex_buffer(&mut self, index: i32, buffer: &Self::BufferType) {
        self.set_vertex_buffer(index, buffer);
    }

    fn set_vertex_state(&mut self, vertex_state: &Self::VertexStateType) {
        self.set_vertex_state(vertex_state);
    }

    fn dispatch(&mut self, count_x: i32, count_y: i32, count_z: i32) {
        self.dispatch(count_x, count_y, count_z);
    }

    fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        self.draw(primitive_topology, vertex_count, vertex_offset);
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
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        index_buffer: &Self::BufferType,
        index_count: i32,
        base_vertex: i32,
    ) {
        self.draw_indexed(
            primitive_topology,
            index_format,
            index_buffer,
            index_count,
            base_vertex,
        );
    }

    fn draw_indexed_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        index_buffer: &Self::BufferType,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        self.draw_indexed_instanced(
            primitive_topology,
            index_format,
            index_buffer,
            index_count,
            base_vertex,
            instance_count,
            base_instance,
        );
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{
        BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IDevice, ShaderInfo,
    };

    use crate::{BufferWgpu, CommandBufferWgpu, DeviceWgpu, ShaderWgpu};

    #[test]
    fn build_compute_command() {
        let device = DeviceWgpu::new(&DeviceInfo::new());

        let shader_source = include_str!("../../resources/tests/simple_compute.glsl");
        let mut compiler = shaderc::Compiler::new().unwrap();
        let shader_binary = compiler
            .compile_into_spirv(
                &shader_source,
                shaderc::ShaderKind::Compute,
                "test.glsl",
                "main",
                None,
            )
            .unwrap();
        let shader = ShaderWgpu::new(
            &device,
            &ShaderInfo::new().set_compute_shader_binary(shader_binary.as_binary_u8()),
        );

        let buffer = BufferWgpu::new(
            &device,
            &BufferInfo::new()
                .set_gpu_access_flags(GpuAccess::UNORDERED_ACCESS_BUFFER)
                .set_size(1024),
        );
        let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

        command_buffer.begin();
        command_buffer.set_shader(&shader);
        command_buffer.set_unordered_access_buffer(0, &buffer);
        command_buffer.dispatch(1, 1, 1);
        command_buffer.end();

        let _ = command_buffer.build_command();
        device.get_device().poll(wgpu::Maintain::Wait);
    }
}
