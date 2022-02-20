use std::sync::Arc;

use sjgfx_interface::{CommandBufferInfo, PrimitiveTopology};
use vulkano::pipeline::Pipeline;
use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer, SubpassContents},
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    device::{Device, Queue},
    format::Format,
    image::ImageViewAbstract,
    pipeline::{
        graphics::{
            input_assembly::InputAssemblyState,
            rasterization::{CullMode, FrontFace, RasterizationState},
            vertex_input::BuffersDefinition,
            viewport::{Viewport, ViewportState},
        },
        ComputePipeline, GraphicsPipeline, PipelineBindPoint,
    },
    render_pass::{Framebuffer, RenderPass, Subpass},
};

use crate::{
    BufferVk, ColorTargetViewVk, DepthStencilViewVk, DeviceVk, Float32_32, ShaderVk, VertexStateVk,
};

pub struct CommandBufferVk<'a> {
    device: Arc<Device>,
    queue: Arc<Queue>,
    shader: Option<&'a ShaderVk>,
    depth_stencil_view: Option<&'a DepthStencilViewVk>,

    // RenderTargets
    render_targets: Option<Vec<Arc<dyn ImageViewAbstract>>>,
    render_target_format: Option<Format>,

    // Buffers
    constant_buffers: [Option<&'a BufferVk>; 64],
    vertex_buffers: [Option<&'a BufferVk>; 64],

    // RenderState
    vertex_state: Option<&'a VertexStateVk>,

    dispatch_count: Option<(u32, u32, u32)>,
    primitive_topology: Option<PrimitiveTopology>,
    vertex_count: Option<i32>,
    vertex_offset: Option<i32>,
    render_pass: Option<Arc<RenderPass>>,
}

impl<'a> CommandBufferVk<'a> {
    pub fn new(device: &'a DeviceVk, _info: &CommandBufferInfo) -> Self {
        Self {
            device: device.clone_device(),
            queue: device.clone_queue(),
            shader: None,
            depth_stencil_view: None,
            render_targets: None,
            render_target_format: None,
            constant_buffers: [None; 64],
            vertex_buffers: [None; 64],
            vertex_state: None,
            dispatch_count: None,
            render_pass: None,
            primitive_topology: None,
            vertex_count: None,
            vertex_offset: None,
        }
    }

    pub fn begin(&mut self) {}

    pub fn end(&mut self) {}

    pub fn set_render_targets_ref<TIterator>(
        &mut self,
        color_target_views: TIterator,
        depth_stencil_view: Option<&'a DepthStencilViewVk>,
    ) where
        TIterator: Iterator<Item = &'a ColorTargetViewVk<'a>>,
    {
        // カラーターゲットをセット
        let mut render_targets = Vec::new();
        for color_target_view in color_target_views {
            render_targets.push(color_target_view.clone_image_view());
            self.render_target_format = Some(color_target_view.get_format());
        }
        self.render_targets = Some(render_targets);

        // TODO: 深度ステンシル
        self.depth_stencil_view = depth_stencil_view;
    }

    pub fn set_render_targets<TIterator>(
        &mut self,
        color_target_views: TIterator,
        depth_stencil_view: Option<&'a DepthStencilViewVk>,
    ) where
        TIterator: Iterator<Item = ColorTargetViewVk<'a>>,
    {
        // カラーターゲットをセット
        let mut render_targets = Vec::new();
        for color_target_view in color_target_views {
            render_targets.push(color_target_view.clone_image_view());
            self.render_target_format = Some(color_target_view.get_format());
        }
        self.render_targets = Some(render_targets);

        // TODO: 深度ステンシル
        self.depth_stencil_view = depth_stencil_view;
    }

    pub fn set_shader(&mut self, shader: &'a ShaderVk) {
        self.shader = Some(shader);
    }

    pub fn set_constant_buffer(&mut self, slot: i32, buffer: &'a BufferVk) {
        self.constant_buffers[slot as usize] = Some(buffer);
    }

    pub fn set_vertex_state(&mut self, vertex_state: &'a VertexStateVk) {
        self.vertex_state = Some(vertex_state);
    }

    pub fn get_vertex_state(&self) -> &'a VertexStateVk {
        self.vertex_state.as_ref().unwrap()
    }

    pub fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        self.primitive_topology = Some(primitive_topology);
        self.vertex_count = Some(vertex_count);
        self.vertex_offset = Some(vertex_offset)
    }

    pub fn get_draw_vertex_count(&self) -> i32 {
        *self.vertex_count.as_ref().unwrap()
    }

    pub fn get_draw_vertex_offset(&self) -> i32 {
        *self.vertex_offset.as_ref().unwrap()
    }

    pub fn dispatch(&mut self, x: u32, y: u32, z: u32) {
        self.dispatch_count = Some((x, y, z));
    }

    pub fn get_depth_stencil_state(&self) -> &Option<&DepthStencilViewVk> {
        &self.depth_stencil_view
    }

    pub fn get_shader(&self) -> &'a ShaderVk {
        self.shader.as_ref().unwrap()
    }

    pub fn get_constant_buffers(&self) -> &[Option<&BufferVk>] {
        &self.constant_buffers
    }

    pub fn set_vertex_buffer(&mut self, index: i32, vertex_buffer: &'a BufferVk) {
        self.vertex_buffers[index as usize] = Some(vertex_buffer);
    }

    pub fn get_vertex_buffers(&self) -> &[Option<&BufferVk>] {
        &self.vertex_buffers
    }

    pub fn get_dispatch_count(&self) -> (u32, u32, u32) {
        self.dispatch_count.as_ref().unwrap().clone()
    }

    pub fn clone_render_pass(&self) -> Arc<RenderPass> {
        self.render_pass.as_ref().unwrap().clone()
    }

    pub(crate) fn build_command_builder(
        &self,
    ) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
        if let Some(shader) = self.shader {
            if shader.is_compute() {
                self.build_compute_command()
            } else {
                self.build_graphics_command()
            }
        } else {
            panic!();
        }
    }

    fn build_compute_command(&self) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
        let shader = self.get_shader().clone_shader();
        let pipeline = ComputePipeline::new(
            self.device.clone(),
            shader.entry_point("main").unwrap(),
            &(),
            None,
            |_| {},
        )
        .unwrap();

        let layout = pipeline.layout().clone();
        let descriptor_set_layout = layout.descriptor_set_layouts().get(0).unwrap();

        let constant_buffer = self.get_constant_buffers()[0]
            .as_ref()
            .unwrap()
            .clone_buffer();
        let set = PersistentDescriptorSet::new(
            descriptor_set_layout.clone(),
            [WriteDescriptorSet::buffer(
                0, /*binding*/
                constant_buffer,
            )],
        )
        .unwrap();

        let mut builder = AutoCommandBufferBuilder::primary(
            self.device.clone(),
            self.queue.as_ref().family(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let (x, y, z) = self.get_dispatch_count();
        builder
            .bind_pipeline_compute(pipeline)
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                layout.clone(),
                0, /*first set*/
                set.clone(),
            )
            .dispatch([x, y, z])
            .unwrap();

        builder
    }

    fn build_graphics_command(&self) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
        let render_pass = vulkano::single_pass_renderpass!(
            self.device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: *self.render_target_format.as_ref().unwrap(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap();

        let vertex_shader = self
            .get_shader()
            .get_vertex_shader()
            .entry_point("main")
            .unwrap();
        let pixel_shader = self
            .get_shader()
            .get_pixel_shader()
            .entry_point("main")
            .unwrap();

        let pipeline = GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().vertex::<Float32_32>())
            .vertex_shader(vertex_shader, ())
            .fragment_shader(pixel_shader, ())
            .rasterization_state(
                RasterizationState::new()
                    .cull_mode(CullMode::Back)
                    .front_face(FrontFace::CounterClockwise),
            )
            .render_pass(Subpass::from(render_pass.clone(), 0 /*id*/).unwrap())
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .input_assembly_state(InputAssemblyState::new())
            .build(self.device.clone())
            .unwrap();

        // TODO
        let vertex_buffer = self.get_vertex_buffers()[0]
            .as_ref()
            .unwrap()
            .clone_vertex_buffer_as::<Float32_32>();

        let framebuffer = {
            let mut builder = Framebuffer::start(render_pass.clone());

            for view in self.render_targets.as_ref().unwrap() {
                builder = builder.add(view.clone()).unwrap();
            }
            builder.build().unwrap()
        };

        let clear_values = vec![[0.0, 0.5, 0.5, 1.0].into()];
        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };
        let mut builder = AutoCommandBufferBuilder::primary(
            self.device.clone(),
            self.queue.as_ref().family(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();
        builder
            .begin_render_pass(framebuffer.clone(), SubpassContents::Inline, clear_values)
            .unwrap()
            .set_viewport(0, [viewport])
            .bind_pipeline_graphics(pipeline)
            .bind_vertex_buffers(0, vertex_buffer)
            .draw(10 /*vertex buffers*/, 1, 0, 0)
            .unwrap()
            .end_render_pass()
            .unwrap();
        builder
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{
        BufferInfo, ColorTargetViewInfo, GpuAccess, IDevice, ImageFormat, PrimitiveTopology,
        ShaderInfo, TextureInfo, VertexStateInfo,
    };
    use sjgfx_interface::{CommandBufferInfo, DeviceInfo};

    use crate::{
        BufferVk, ColorTargetViewVk, CommandBufferVk, DeviceVk, Float32_32, ShaderVk, TextureVk,
        VertexStateVk,
    };

    #[test]
    fn command_builder_test() {
        let vertex_shader_source = "
				#version 450
				layout(location = 0) in vec2 i_Position;
				void main() {
					gl_Position = vec4(i_Position, 0.0, 1.0);
				}";
        let pixel_shader_source = "
				#version 450
				layout(location = 0) out vec4 o_Color;
				void main() {
					o_Color = vec4(1.0, 0.0, 0.0, 1.0);
				}";
        let mut compiler = shaderc::Compiler::new().unwrap();
        let vertex_shader_binary = compiler
            .compile_into_spirv(
                &vertex_shader_source,
                shaderc::ShaderKind::Vertex,
                "test.glsl",
                "main",
                None,
            )
            .unwrap();
        let pixel_shader_binary = compiler
            .compile_into_spirv(
                &pixel_shader_source,
                shaderc::ShaderKind::Fragment,
                "test.glsl",
                "main",
                None,
            )
            .unwrap();

        let device = DeviceVk::new(&DeviceInfo::new());
        let mut command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());
        let shader = ShaderVk::new(
            &device,
            &ShaderInfo::new()
                .set_vertex_shader_binary(vertex_shader_binary.as_binary_u8())
                .set_pixel_shader_binary(pixel_shader_binary.as_binary_u8()),
        );
        let vertex_state = VertexStateVk::new(&device, &VertexStateInfo::new());
        let vertex_buffer = BufferVk::new_as_array::<Float32_32>(&device, &BufferInfo::new());

        let texture = TextureVk::new(
            &device,
            &TextureInfo::new()
                .set_width(640)
                .set_height(480)
                .set_gpu_access_flags(GpuAccess::COLOR_BUFFER)
                .set_image_format(ImageFormat::R8G8B8A8Unorm),
        );
        let color_target_view = ColorTargetViewVk::new(
            &device,
            &ColorTargetViewInfo::new().set_image_format(ImageFormat::R8G8B8A8Unorm),
            &texture,
        );

        command_buffer.begin();
        command_buffer.set_render_targets_ref([&color_target_view].into_iter(), None);
        command_buffer.set_shader(&shader);
        command_buffer.set_vertex_state(&vertex_state);
        command_buffer.set_vertex_buffer(0, &vertex_buffer);
        command_buffer.draw(
            PrimitiveTopology::TriangleList,
            3, /*vertex_count*/
            0, /*vertex_offset*/
        );
        command_buffer.end();

        command_buffer.build_command_builder();
    }
}
