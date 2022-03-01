use sjgfx_interface::{CommandBufferInfo, PrimitiveTopology};

use crate::{BufferWgpu, ColorTargetViewWgpu, DeviceWgpu, ShaderWgpu};

pub struct CommandBufferWgpu<'a> {
    device: &'a DeviceWgpu,

    // ColorTargetView
    color_target_view: Option<ColorTargetViewWgpu>,

    shader: Option<&'a ShaderWgpu>,
    constant_buffers: [Option<&'a BufferWgpu<'a>>; 64],
    unordered_access_buffer: [Option<&'a BufferWgpu<'a>>; 64],
    dispatch_count: Option<(u32, u32, u32)>,

    // Draw
    primitive_topology: Option<PrimitiveTopology>,
    vertex_count: Option<u32>,
    vertex_offset: Option<i32>,
}

impl<'a> CommandBufferWgpu<'a> {
    pub fn new(device: &'a DeviceWgpu, _info: &CommandBufferInfo) -> Self {
        Self {
            device,
            color_target_view: None,
            shader: None,
            constant_buffers: [None; 64],
            unordered_access_buffer: [None; 64],
            dispatch_count: None,
            primitive_topology: None,
            vertex_count: None,
            vertex_offset: None,
        }
    }

    pub fn begin(&self) {}

    pub fn end(&self) {}

    pub fn set_render_targets<TIterator>(&mut self, mut color_target_views: TIterator)
    where
        TIterator: Iterator<Item = ColorTargetViewWgpu>,
    {
        if let Some(color_target_view) = color_target_views.next() {
            self.color_target_view = Some(color_target_view);
        }
    }

    pub fn set_shader(&mut self, shader: &'a ShaderWgpu) {
        self.shader = Some(shader);
    }

    pub fn set_constant_buffer(&mut self, index: i32, buffer: &'a BufferWgpu) {
        self.constant_buffers[index as usize] = Some(buffer);
    }

    pub fn set_unordered_access_buffer(&mut self, index: i32, buffer: &'a BufferWgpu) {
        self.unordered_access_buffer[index as usize] = Some(buffer);
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
        self.primitive_topology = Some(primitive_topology);
        self.vertex_count = Some(vertex_count as u32);
        self.vertex_offset = Some(vertex_offset);
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
        let render_pipeline =
            self.device
                .get_device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: None,
                    vertex: wgpu::VertexState {
                        module: &vertex_shader_module,
                        entry_point: "main",
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &pixel_shader_module,
                        entry_point: "main",
                        targets: &[color_target_view.get_texture_format().into()],
                    }),
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None,
                });

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
                depth_stencil_attachment: None,
            });

            // パイプライン
            render_pass.set_pipeline(&render_pipeline);

            // 描画
            if let Some(vertex_count) = self.vertex_count {
                render_pass.draw(0..vertex_count, 0..1);
            }
        }
        command_encoder.finish()
    }

    fn create_bind_group(&self) -> wgpu::BindGroup {
        self.device
            .get_device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: self.shader.as_ref().unwrap().get_bind_group_layout(),
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.unordered_access_buffer[0]
                        .as_ref()
                        .unwrap()
                        .get_buffer()
                        .as_entire_binding(),
                }],
            })
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
