use std::sync::Arc;

use wgpu::util::DeviceExt;

use crate::ColorTargetViewWgpu;

pub struct SwapChainPipeline {
    device: Arc<wgpu::Device>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    #[allow(dead_code)]
    sampler: wgpu::Sampler,
    #[allow(dead_code)]
    texture: wgpu::Texture,

    width: u32,
    height: u32,

    color_target_view: ColorTargetViewWgpu,
}

impl SwapChainPipeline {
    pub fn new(device: Arc<wgpu::Device>, texture_format: wgpu::TextureFormat) -> Self {
        let vertex_shader_source = include_str!("../../resources/render_color_target.vs");
        let pixel_shader_source = include_str!("../../resources/render_color_target.fs");
        let mut compiler = sjgfx_util::ShaderCompiler::new();
        let vertex_shader_binary =
            compiler.create_binary(&vertex_shader_source, sjgfx_util::ShaderStage::Vertex);
        let pixel_shader_binary =
            compiler.create_binary(&pixel_shader_source, sjgfx_util::ShaderStage::Pixel);

        let vertex_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::util::make_spirv(&vertex_shader_binary),
        });
        let pixel_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::util::make_spirv(&pixel_shader_binary),
        });
        let buffers = &[wgpu::VertexBufferLayout {
            array_stride: (std::mem::size_of::<f32>() * 4) as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: (std::mem::size_of::<f32>() * 2) as u64,
                    shader_location: 1,
                },
            ],
        }];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &vertex_module,
                entry_point: "main",
                buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &pixel_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: texture_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[
                -1.0f32, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                0.0,
            ]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[0u32, 1, 2, 0, 2, 3]),
            usage: wgpu::BufferUsages::INDEX,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: 1280,
                height: 960,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let bind_group_entries = [
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ];
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &bind_group_entries,
        });

        Self {
            device,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            sampler,
            texture,
            color_target_view: ColorTargetViewWgpu::new_direct(
                Arc::new(texture_view),
                wgpu::TextureFormat::Rgba8Unorm,
            ),
            bind_group,
            width: 1280,
            height: 960,
        }
    }

    pub fn build_command(&self, texture_view: &wgpu::TextureView) -> wgpu::CommandBuffer {
        let mut command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        let color_attachment = wgpu::RenderPassColorAttachment {
            view: texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        };
        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[color_attachment],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(
                0..6, /*indices*/
                0,    /*base_vertex*/
                0..1, /*instance*/
            );
        }
        command_encoder.finish()
    }

    pub fn get_color_target_view(&self) -> &ColorTargetViewWgpu {
        &self.color_target_view
    }

    pub fn get_color_target_view_mut(&mut self) -> &mut ColorTargetViewWgpu {
        &mut self.color_target_view
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
