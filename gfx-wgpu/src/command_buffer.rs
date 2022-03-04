use sjgfx_interface::{CommandBufferInfo, IndexFormat, PrimitiveTopology};

use crate::{
    BufferWgpu, ColorTargetViewWgpu, DepthStencilViewWgpu, DeviceWgpu, GpuAddressWgpu, ShaderWgpu,
    VertexStateWgpu,
};

struct DrawInfo {
    #[allow(dead_code)]
    pub primitive_topology: PrimitiveTopology,
    pub vertex_count: u32,
}

struct DrawIndexedInfo<'a> {
    #[allow(dead_code)]
    pub primitive_topology: PrimitiveTopology,
    pub index_format: wgpu::IndexFormat,
    pub index_buffer: &'a BufferWgpu<'a>,
    pub index_count: u32,
}

enum DrawCommand<'a> {
    Draw(DrawInfo),
    DrawIndexed(DrawIndexedInfo<'a>),
}

pub struct CommandBufferWgpu<'a> {
    device: &'a DeviceWgpu,

    // レンダーターゲット
    color_target_view: Option<ColorTargetViewWgpu>,
    depth_stencil_view: Option<&'a DepthStencilViewWgpu<'a>>,

    shader: Option<&'a ShaderWgpu>,
    constant_buffers: [Option<&'a BufferWgpu<'a>>; 64],
    constant_buffer_addresses: [Option<GpuAddressWgpu<'a>>; 8],
    unordered_access_buffer: [Option<&'a BufferWgpu<'a>>; 64],
    dispatch_count: Option<(u32, u32, u32)>,

    // Draw
    vertex_buffer: [Option<&'a BufferWgpu<'a>>; 64],
    vertex_state: Option<&'a VertexStateWgpu>,
    draw_command: Option<DrawCommand<'a>>,
}

impl<'a> CommandBufferWgpu<'a> {
    pub fn new(device: &'a DeviceWgpu, _info: &CommandBufferInfo) -> Self {
        Self {
            device,
            color_target_view: None,
            depth_stencil_view: None,
            shader: None,
            constant_buffers: [None; 64],
            constant_buffer_addresses: [None, None, None, None, None, None, None, None],
            unordered_access_buffer: [None; 64],
            dispatch_count: None,
            vertex_buffer: [None; 64],
            vertex_state: None,
            draw_command: None,
        }
    }

    pub fn begin(&self) {}

    pub fn end(&self) {}

    pub fn set_render_targets<TIterator>(
        &mut self,
        mut color_target_views: TIterator,
        depth_stencil_view: Option<&'a DepthStencilViewWgpu<'a>>,
    ) where
        TIterator: Iterator<Item = ColorTargetViewWgpu>,
    {
        if let Some(color_target_view) = color_target_views.next() {
            self.color_target_view = Some(color_target_view);
        }

        self.depth_stencil_view = depth_stencil_view;
    }

    pub fn set_shader(&mut self, shader: &'a ShaderWgpu) {
        self.shader = Some(shader);
    }

    pub fn set_constant_buffer(&mut self, index: i32, buffer: &'a BufferWgpu) {
        self.constant_buffers[index as usize] = Some(buffer);
    }

    pub fn set_constant_buffer_address(&mut self, index: i32, gpu_address: GpuAddressWgpu<'a>) {
        self.constant_buffer_addresses[index as usize] = Some(gpu_address);
    }

    pub fn set_unordered_access_buffer(&mut self, index: i32, buffer: &'a BufferWgpu) {
        self.unordered_access_buffer[index as usize] = Some(buffer);
    }

    pub fn set_vertex_buffer(&mut self, index: i32, buffer: &'a BufferWgpu) {
        self.vertex_buffer[index as usize] = Some(buffer);
    }

    pub fn set_vertex_state(&mut self, vertex_state: &'a VertexStateWgpu) {
        self.vertex_state = Some(vertex_state);
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
        _vertex_offset: i32,
    ) {
        let draw_info = DrawInfo {
            primitive_topology,
            vertex_count: vertex_count as u32,
        };
        self.draw_command = Some(DrawCommand::Draw(draw_info));
    }

    pub fn draw_indexed(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        index_buffer: &'a BufferWgpu,
        index_count: i32,
        _base_vertex: i32,
    ) {
        let index_format_wgpu = match index_format {
            IndexFormat::Uint32 => wgpu::IndexFormat::Uint32,
        };

        let draw_indexed_info = DrawIndexedInfo {
            primitive_topology,
            index_format: index_format_wgpu,
            index_buffer,
            index_count: index_count as u32,
        };
        self.draw_command = Some(DrawCommand::DrawIndexed(draw_indexed_info));
    }

    pub(crate) fn build_command(&self) -> Option<wgpu::CommandBuffer> {
        if let Some(shader) = self.shader {
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
            .get_device()
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
        let vertex_buffer_layout = if let Some(vertex_state) = self.vertex_state {
            let attributes = self.shader.as_ref().unwrap().get_vertex_attributes();
            vertex_state.create_vertex_buffer_layout(attributes)
        } else {
            vec![]
        };

        let render_pipeline =
            self.device
                .get_device()
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
            .get_device()
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

            // 頂点バッファ
            if let Some(vertex_buffer) = self.vertex_buffer[0] {
                render_pass.set_vertex_buffer(0, vertex_buffer.get_buffer().slice(..));
            }

            // 描画
            if let Some(draw_command) = &self.draw_command {
                match draw_command {
                    DrawCommand::Draw(ref draw_info) => {
                        render_pass.draw(0..draw_info.vertex_count, 0..1);
                    }
                    DrawCommand::DrawIndexed(ref draw_indexed_info) => {
                        let buffer_slice = draw_indexed_info.index_buffer.get_buffer().slice(..);
                        render_pass.set_index_buffer(buffer_slice, draw_indexed_info.index_format);
                        render_pass.draw_indexed(0..draw_indexed_info.index_count, 0, 0..1);
                    }
                }
            }
        }
        command_encoder.finish()
    }

    fn create_bind_group(&self) -> wgpu::BindGroup {
        if let Some(unordered_access_buffer) = self.unordered_access_buffer[0] {
            self.device
                .get_device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: self.shader.as_ref().unwrap().get_bind_group_layout(),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: unordered_access_buffer.get_buffer().as_entire_binding(),
                    }],
                })
        } else if let Some(gpu_address) = &self.constant_buffer_addresses[0] {
            self.device
                .get_device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: self.shader.as_ref().unwrap().get_bind_group_layout(),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: gpu_address.get_binding_resource(),
                    }],
                })
        } else {
            self.device
                .get_device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: self.shader.as_ref().unwrap().get_bind_group_layout(),
                    entries: &[],
                })
        }
    }

    fn create_depth_stencil_state(&self) -> Option<wgpu::DepthStencilState> {
        if let Some(_depth_stencil_view) = self.depth_stencil_view {
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
        if let Some(depth_stencil_view) = self.depth_stencil_view {
            Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_stencil_view.get_texture_view(),
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
