use std::sync::Arc;

use sjgfx_interface::{
    CommandBufferInfo, ICommandBuffer, PrimitiveTopology, ScissorStateInfo, TextureArrayRange,
    ViewportScissorStateInfo, ViewportStateInfo,
};
use vulkano::command_buffer::allocator::{
    CommandBufferAllocator, StandardCommandBufferAllocator,
    StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::command_buffer::{RenderPassBeginInfo, SubpassContents};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::pipeline::Pipeline;
use vulkano::render_pass::FramebufferCreateInfo;
use vulkano::shader::ShaderModule;
use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    device::{Device, Queue},
    format::Format,
    image::ImageViewAbstract,
    pipeline::{
        graphics::{
            input_assembly::InputAssemblyState,
            rasterization::{CullMode, FrontFace, RasterizationState},
            //vertex_input::BuffersDefinition,
            viewport::ViewportState,
        },
        ComputePipeline, GraphicsPipeline, PipelineBindPoint,
    },
    render_pass::{Framebuffer, RenderPass, Subpass},
};

use crate::buffer_vk::BufferView;
use crate::vertex_state_vk::VertexStateView;
use crate::viewport_scissor_state_vk::ViewportScissorStateView;
use crate::{
    BufferVk, ColorTargetViewVk, DepthStencilViewVk, DeviceVk, SamplerVk, ShaderVk, TextureViewVk,
    TextureVk, VertexStateVk, ViewportScissorStateVk,
};

struct DrawInfo {
    #[allow(dead_code)]
    pub primitive_topology: PrimitiveTopology,
    pub vertex_count: u32,
    #[allow(dead_code)]
    pub vertex_offset: u32,
}

struct DrawIndexedInfo {
    pub index_count: u32,
    pub instance_count: u32,
    pub vertex_offset: i32,
    pub index_buffer: Arc<BufferView>,
}

enum DrawCommand {
    Draw(DrawInfo),
    #[allow(dead_code)]
    DrawIndexed(DrawIndexedInfo),
}

pub struct CommandBufferVk {
    device: Arc<Device>,
    queue: Arc<Queue>,
    command_buffer_allocator: StandardCommandBufferAllocator,
    descriptor_set_allocator: StandardDescriptorSetAllocator,

    // シェーダ
    compute_shader_module: Option<Arc<ShaderModule>>,
    vertex_shader_module: Option<Arc<ShaderModule>>,
    pixel_shader_module: Option<Arc<ShaderModule>>,

    // RenderTargets
    render_targets: Option<Vec<Arc<dyn ImageViewAbstract>>>,
    render_target_format: Option<Format>,
    depth_stencil_view: Option<()>,

    // Buffers
    constant_buffers: [Option<BufferView>; 8],
    vertex_buffers: [Option<BufferView>; 8],
    unordered_access_buffer: [Option<BufferView>; 8],

    // RenderState
    viewport_scissor_state: Option<ViewportScissorStateView>,
    vertex_state: Option<VertexStateView>,

    dispatch_count: Option<(u32, u32, u32)>,

    // 描画
    draw_command: Option<DrawCommand>,
    render_pass: Option<Arc<RenderPass>>,
}

impl CommandBufferVk {
    pub fn new(device: &DeviceVk, _info: &CommandBufferInfo) -> Self {
        let command_buffer_allocator = StandardCommandBufferAllocator::new(
            device.clone_device(),
            StandardCommandBufferAllocatorCreateInfo {
                primary_buffer_count: 1,
                secondary_buffer_count: 1,
                ..Default::default()
            },
        );
        let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone_device());
        let viewport_scissor_state = ViewportScissorStateVk::new(
            device,
            &ViewportScissorStateInfo::new()
                .set_viewport_state_info_array(&[ViewportStateInfo::new()
                    .set_width(640.0)
                    .set_height(480.0)])
                .set_scissor_state_info_array(&[ScissorStateInfo::new()
                    .set_width(640)
                    .set_height(480)]),
        );

        Self {
            device: device.clone_device(),
            queue: device.clone_queue(),
            command_buffer_allocator,
            descriptor_set_allocator,

            // シェーダ
            compute_shader_module: None,
            vertex_shader_module: None,
            pixel_shader_module: None,

            depth_stencil_view: None,
            render_targets: None,
            render_target_format: None,

            // バッファ
            constant_buffers: [None, None, None, None, None, None, None, None],
            vertex_buffers: [None, None, None, None, None, None, None, None],
            unordered_access_buffer: std::default::Default::default(),

            // RenderState
            viewport_scissor_state: Some(viewport_scissor_state.view()),
            vertex_state: None,

            dispatch_count: None,
            render_pass: None,
            draw_command: None,
        }
    }

    pub fn begin(&mut self) {}

    pub fn end(&mut self) {}

    pub fn set_render_targets_ref<'a, TIterator>(
        &mut self,
        color_target_views: TIterator,
        _depth_stencil_view: Option<&DepthStencilViewVk>,
    ) where
        TIterator: Iterator<Item = &'a ColorTargetViewVk>,
    {
        // カラーターゲットをセット
        let mut render_targets = Vec::new();
        for color_target_view in color_target_views {
            render_targets.push(color_target_view.clone_image_view());
            self.render_target_format = Some(color_target_view.get_format());
        }
        self.render_targets = Some(render_targets);

        // TODO: 深度ステンシル
        self.depth_stencil_view = None;
    }

    pub fn set_render_targets(
        &mut self,
        color_target_views: &[&ColorTargetViewVk],
        _depth_stencil_view: Option<&DepthStencilViewVk>,
    ) {
        // カラーターゲットをセット
        let mut render_targets = Vec::new();
        for color_target_view in color_target_views {
            render_targets.push(color_target_view.clone_image_view());
            self.render_target_format = Some(color_target_view.get_format());
        }
        self.render_targets = Some(render_targets);

        // TODO: 深度ステンシル
        self.depth_stencil_view = None;
    }

    pub fn set_viewport_scissor_state(&mut self, viewport_scissor_state: &ViewportScissorStateVk) {
        self.viewport_scissor_state = Some(viewport_scissor_state.view());
    }

    pub fn set_shader(&mut self, shader: &ShaderVk) {
        self.compute_shader_module = shader.acquire_compute_shader_module();
        self.vertex_shader_module = shader.acquire_vertex_shader_module();
        self.pixel_shader_module = shader.acquire_pixel_shader_module();
    }

    pub fn set_constant_buffer(&mut self, slot: i32, buffer: &BufferVk) {
        self.constant_buffers[slot as usize] = Some(buffer.view());
    }

    pub fn set_unordered_access_buffer(&mut self, slot: i32, buffer: &BufferVk) {
        let index = slot as usize;
        self.unordered_access_buffer[index] = Some(buffer.view());
    }

    pub fn set_vertex_state(&mut self, vertex_state: &VertexStateVk) {
        self.vertex_state = Some(vertex_state.view());
    }

    pub fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        let draw_info = DrawInfo {
            primitive_topology,
            vertex_count: vertex_count as u32,
            vertex_offset: vertex_offset as u32,
        };
        let draw_command = DrawCommand::Draw(draw_info);
        self.draw_command = Some(draw_command);
    }

    fn draw_indexed(
        &mut self,
        _primitive_topology: PrimitiveTopology,
        _index_format: sjgfx_interface::IndexFormat,
        index_buffer: &BufferVk,
        index_count: i32,
        base_vertex: i32,
    ) {
        let draw_info = DrawIndexedInfo {
            index_count: index_count as u32,
            instance_count: 1,
            vertex_offset: base_vertex,
            index_buffer: Arc::new(index_buffer.view()),
        };
        let draw_command = DrawCommand::DrawIndexed(draw_info);
        self.draw_command = Some(draw_command);
    }

    pub fn dispatch(&mut self, x: u32, y: u32, z: u32) {
        self.dispatch_count = Some((x, y, z));
    }

    pub fn set_vertex_buffer(&mut self, index: i32, vertex_buffer: &BufferVk) {
        self.vertex_buffers[index as usize] = Some(vertex_buffer.view());
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
        let mut builder = AutoCommandBufferBuilder::primary(
            &self.command_buffer_allocator,
            self.queue.queue_family_index(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        if self.compute_shader_module.is_some() {
            self.build_compute_command(&mut builder);
        } else if self.vertex_shader_module.is_some() {
            self.build_graphics_command(&mut builder);
        } else {
            // とくに何もしない
        }

        builder
    }

    fn build_compute_command<L, A>(&self, builder: &mut AutoCommandBufferBuilder<L, A>)
    where
        A: CommandBufferAllocator,
    {
        let shader = self.compute_shader_module.as_ref().unwrap().clone();
        let pipeline = ComputePipeline::new(
            self.device.clone(),
            shader.entry_point("main").unwrap(),
            &(),
            None,
            |_| {},
        )
        .unwrap();

        self.push_descriptors(builder, PipelineBindPoint::Compute, pipeline.as_ref());

        let (x, y, z) = self.get_dispatch_count();
        builder
            .bind_pipeline_compute(pipeline)
            .dispatch([x, y, z])
            .unwrap();
    }

    fn build_graphics_command<L, A>(&self, builder: &mut AutoCommandBufferBuilder<L, A>)
    where
        A: CommandBufferAllocator,
    {
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
            .vertex_shader_module
            .as_ref()
            .unwrap()
            .entry_point("main")
            .unwrap();
        let pixel_shader = self
            .pixel_shader_module
            .as_ref()
            .unwrap()
            .entry_point("main")
            .unwrap();

        let pipeline = GraphicsPipeline::start()
            .vertex_input_state(self.vertex_state.as_ref().unwrap().clone())
            .vertex_shader(vertex_shader, ())
            .fragment_shader(pixel_shader, ())
            .rasterization_state(
                RasterizationState::new()
                    .cull_mode(CullMode::None)
                    .front_face(FrontFace::Clockwise),
            )
            .render_pass(Subpass::from(render_pass.clone(), 0 /*id*/).unwrap())
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .input_assembly_state(InputAssemblyState::new())
            .build(self.device.clone())
            .unwrap();

        let vertex_buffer = self.vertex_buffers[0].as_ref().unwrap().clone();

        let views = self.render_targets.as_ref().unwrap();
        let framebuffer = Framebuffer::new(
            render_pass,
            FramebufferCreateInfo {
                attachments: views.to_vec(),
                ..Default::default()
            },
        )
        .unwrap();

        let clear_values = vec![Some([0.0, 0.5, 0.5, 1.0].into())];

        self.push_descriptors(builder, PipelineBindPoint::Graphics, pipeline.as_ref());
        self.push_viewports_and_scissors(builder);

        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values,
                    ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                },
                SubpassContents::Inline,
            )
            .unwrap()
            .bind_pipeline_graphics(pipeline)
            .bind_vertex_buffers(0, vertex_buffer.buffer.clone());

        // 描画コマンドを積む順番大事
        // パイプラインが設定されてないとコマンド追加に失敗する
        self.push_draw_command(builder);
        builder.end_render_pass().unwrap();
    }

    fn push_descriptors<TPipeline, L, A>(
        &self,
        command_builder: &mut AutoCommandBufferBuilder<L, A>,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: &TPipeline,
    ) where
        TPipeline: Pipeline,
        A: CommandBufferAllocator,
    {
        if let Some(descriptor_sets) = self.create_descriptor_sets(pipeline) {
            let pipeline_layout = pipeline.layout().clone();
            command_builder.bind_descriptor_sets(
                pipeline_bind_point,
                pipeline_layout,
                0, /*first_set*/
                descriptor_sets,
            );
        }
    }

    fn push_viewports_and_scissors<L, A>(
        &self,
        command_builder: &mut AutoCommandBufferBuilder<L, A>,
    ) where
        A: CommandBufferAllocator,
    {
        if let Some(viewport_scissor_state) = &self.viewport_scissor_state {
            command_builder.set_viewport(0, viewport_scissor_state.viewports.to_vec());
            command_builder.set_scissor(0, viewport_scissor_state.scissors.to_vec());
        }
    }

    fn push_draw_command<L, A>(&self, command_builder: &mut AutoCommandBufferBuilder<L, A>)
    where
        A: CommandBufferAllocator,
    {
        if let Some(draw_command) = &self.draw_command {
            match draw_command {
                DrawCommand::Draw(ref info) => {
                    command_builder.draw(info.vertex_count, 1, 0, 0).unwrap();
                }
                DrawCommand::DrawIndexed(ref info) => {
                    command_builder
                        .bind_index_buffer(info.index_buffer.buffer.clone())
                        .draw_indexed(
                            info.index_count,
                            info.instance_count,
                            0, /*first_index*/
                            info.vertex_offset,
                            0, /*first_instance*/
                        )
                        .unwrap();
                }
            }
        }
    }

    fn create_descriptor_sets<T: Pipeline>(
        &self,
        pipeline: &T,
    ) -> Option<Arc<PersistentDescriptorSet>> {
        let layout = pipeline.layout().clone();
        let descriptor_set_layout = layout.set_layouts().get(0)?;

        let mut write_descriptor_sets = Vec::new();

        // 定数バッファ
        for index in 0..self.constant_buffers.len() {
            if let Some(buffer) = &self.constant_buffers[index] {
                write_descriptor_sets.push(WriteDescriptorSet::buffer(
                    index as u32,
                    buffer.buffer.clone(),
                ));
            }
        }

        // Unordered Access Buffer
        for index in 0..self.unordered_access_buffer.len() {
            if let Some(buffer) = &self.unordered_access_buffer[index] {
                write_descriptor_sets.push(WriteDescriptorSet::buffer(
                    index as u32,
                    buffer.buffer.clone(),
                ));
            }
        }

        let set = PersistentDescriptorSet::new(
            &self.descriptor_set_allocator,
            descriptor_set_layout.clone(),
            write_descriptor_sets,
        )
        .unwrap();
        Some(set)
    }
}

impl ICommandBuffer for CommandBufferVk {
    type DeviceType = DeviceVk;
    type BufferType = BufferVk;
    type ColorTargetViewType = ColorTargetViewVk;
    type DepthStencilViewType = DepthStencilViewVk;
    type SamplerType = SamplerVk;
    type ShaderType = ShaderVk;
    type TextureType = TextureVk;
    type TextureViewType = TextureViewVk;
    type VertexStateType = VertexStateVk;

    fn new(device: &Self::DeviceType, info: &CommandBufferInfo) -> Self {
        Self::new(device, info)
    }

    fn begin(&mut self) {
        self.begin();
    }

    fn end(&mut self) {
        self.end();
    }

    fn clear_color(
        &mut self,
        _color_target_view: &mut Self::ColorTargetViewType,
        _red: f32,
        _green: f32,
        _blue: f32,
        _alpha: f32,
        _texture_array_range: TextureArrayRange,
    ) {
        todo!()
    }

    fn set_render_targets(
        &mut self,
        color_target_views: &[&ColorTargetViewVk],
        depth_stencil_view: Option<&Self::DepthStencilViewType>,
    ) {
        self.set_render_targets(color_target_views, depth_stencil_view)
    }

    fn set_shader(&mut self, shader: &Self::ShaderType) {
        self.set_shader(shader);
    }

    fn set_sampler(&mut self, _index: i32, _sampler: &Self::SamplerType) {
        todo!()
    }

    fn set_texture(&mut self, _index: i32, _texture: &Self::TextureViewType) {
        todo!()
    }

    fn set_image(&mut self, _index: i32, _texture: &Self::TextureViewType) {
        todo!()
    }

    fn set_constant_buffer(&mut self, index: i32, buffer: &Self::BufferType) {
        self.set_constant_buffer(index, buffer);
    }

    fn set_unordered_access_buffer(&mut self, index: i32, buffer: &Self::BufferType) {
        self.set_unordered_access_buffer(index, buffer)
    }

    fn set_vertex_buffer(&mut self, index: i32, buffer: &Self::BufferType) {
        self.set_vertex_buffer(index, buffer);
    }

    fn set_vertex_state(&mut self, vertex_state: &Self::VertexStateType) {
        self.set_vertex_state(vertex_state);
    }

    fn set_scissor(&mut self, _scissor_state_info: &ScissorStateInfo) {
        todo!()
    }

    fn dispatch(&mut self, count_x: i32, count_y: i32, count_z: i32) {
        self.dispatch(count_x as u32, count_y as u32, count_z as u32);
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
        index_format: sjgfx_interface::IndexFormat,
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

// #[cfg(test)]
// mod tests {
//     use sjgfx_interface::{
//         BufferInfo, ColorTargetViewInfo, GpuAccess, IDevice, ImageFormat, PrimitiveTopology,
//         ShaderInfo, TextureInfo, VertexStateInfo,
//     };
//     use sjgfx_interface::{CommandBufferInfo, DeviceInfo};

//     use crate::{
//         BufferVk, ColorTargetViewVk, CommandBufferVk, DeviceVk, Float32_32, ShaderVk, TextureVk,
//         VertexStateVk,
//     };

//     #[test]
//     fn command_builder_test() {
//         let vertex_shader_source = "
// 				#version 450
// 				layout(location = 0) in vec2 i_Position;
// 				void main() {
// 					gl_Position = vec4(i_Position, 0.0, 1.0);
// 				}";
//         let pixel_shader_source = "
// 				#version 450
// 				layout(location = 0) out vec4 o_Color;
// 				void main() {
// 					o_Color = vec4(1.0, 0.0, 0.0, 1.0);
// 				}";
//         let mut compiler = shaderc::Compiler::new().unwrap();
//         let vertex_shader_binary = compiler
//             .compile_into_spirv(
//                 &vertex_shader_source,
//                 shaderc::ShaderKind::Vertex,
//                 "test.glsl",
//                 "main",
//                 None,
//             )
//             .unwrap();
//         let pixel_shader_binary = compiler
//             .compile_into_spirv(
//                 &pixel_shader_source,
//                 shaderc::ShaderKind::Fragment,
//                 "test.glsl",
//                 "main",
//                 None,
//             )
//             .unwrap();

//         let device = DeviceVk::new(&DeviceInfo::new());
//         let mut command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());
//         let shader = ShaderVk::new(
//             &device,
//             &ShaderInfo::new()
//                 .set_vertex_shader_binary(vertex_shader_binary.as_binary_u8())
//                 .set_pixel_shader_binary(pixel_shader_binary.as_binary_u8()),
//         );
//         let vertex_state = VertexStateVk::new(&device, &VertexStateInfo::new());
//         let vertex_buffer = BufferVk::new_as_array::<Float32_32>(&device, &BufferInfo::new());

//         let texture = TextureVk::new(
//             &device,
//             &TextureInfo::new()
//                 .set_width(640)
//                 .set_height(480)
//                 .set_gpu_access_flags(GpuAccess::COLOR_BUFFER)
//                 .set_image_format(ImageFormat::R8G8B8A8Unorm),
//         );
//         let color_target_view = ColorTargetViewVk::new(
//             &device,
//             &ColorTargetViewInfo::new().set_image_format(ImageFormat::R8G8B8A8Unorm),
//             &texture,
//         );

//         command_buffer.begin();
//         command_buffer.set_render_targets_ref([&color_target_view].into_iter(), None);
//         command_buffer.set_shader(&shader);
//         command_buffer.set_vertex_state(&vertex_state);
//         command_buffer.set_vertex_buffer(0, &vertex_buffer);
//         command_buffer.draw(
//             PrimitiveTopology::TriangleList,
//             3, /*vertex_count*/
//             0, /*vertex_offset*/
//         );
//         command_buffer.end();

//         command_buffer.build_command_builder();
//     }
// }
