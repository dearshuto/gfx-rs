use std::sync::Arc;

use sjgfx_interface::{
    CommandBufferInfo, ICommandBuffer, PrimitiveTopology, TextureArrayRange,
    VertexAttributeStateInfo, VertexBufferStateInfo,
};
use vulkano::format::ClearValue;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::Pipeline;
use vulkano::shader::ShaderModule;
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
            //vertex_input::BuffersDefinition,
            viewport::{Viewport, ViewportState},
        },
        ComputePipeline, GraphicsPipeline, PipelineBindPoint,
    },
    render_pass::{Framebuffer, RenderPass, Subpass},
};

use crate::buffer_vk::BufferView;
use crate::{
    BufferVk, ColorTargetViewVk, DepthStencilViewVk, DeviceVk, Float32_32, SamplerVk, ShaderVk,
    TextureViewVk, TextureVk, VertexStateVk,
};

pub struct CommandBufferVk {
    device: Arc<Device>,
    queue: Arc<Queue>,

    // シェーダ
    compute_shader_module: Option<Arc<ShaderModule>>,
    vertex_shader_module: Option<Arc<ShaderModule>>,
    pixel_shader_module: Option<Arc<ShaderModule>>,

    // RenderTargets
    render_targets: Option<Vec<Arc<dyn ImageViewAbstract>>>,
    render_target_format: Option<Format>,
    depth_stencil_view: Option<()>,
    clear_color: [f32; 4],

    // Buffers
    constant_buffers: [Option<BufferView>; 8],
    vertex_buffers: [Option<BufferView>; 8],
    unordered_access_buffer: [Option<BufferView>; 8],

    // RenderState
    attribute_state_infos: Option<Vec<VertexAttributeStateInfo>>,
    buffer_state_infos: Option<Vec<VertexBufferStateInfo>>,

    dispatch_count: Option<(u32, u32, u32)>,
    primitive_topology: Option<PrimitiveTopology>,
    vertex_count: Option<u32>,
    vertex_offset: Option<i32>,
    render_pass: Option<Arc<RenderPass>>,
}

impl CommandBufferVk {
    pub fn new(device: &DeviceVk, _info: &CommandBufferInfo) -> Self {
        Self {
            device: device.clone_device(),
            queue: device.clone_queue(),

            // シェーダ
            compute_shader_module: None,
            vertex_shader_module: None,
            pixel_shader_module: None,

            depth_stencil_view: None,
            render_targets: None,
            render_target_format: None,
            clear_color: [0.0, 0.0, 0.0, 0.0],

            // バッファ
            constant_buffers: [None, None, None, None, None, None, None, None],
            vertex_buffers: [None, None, None, None, None, None, None, None],
            unordered_access_buffer: std::default::Default::default(),

            // RenderState
            attribute_state_infos: None,
            buffer_state_infos: None,

            dispatch_count: None,
            render_pass: None,
            primitive_topology: None,
            vertex_count: None,
            vertex_offset: None,
        }
    }

    pub fn begin(&mut self) {}

    pub fn end(&mut self) {}

    pub fn clear_color(&mut self, _color_target: &mut ColorTargetViewVk, red: f32, green: f32, blue: f32, alpha: f32, _texture_array_range: TextureArrayRange) {
        self.clear_color = [red, green, blue, alpha];
    }

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

    pub fn set_render_targets<TIterator>(
        &mut self,
        color_target_views: TIterator,
        _depth_stencil_view: Option<&DepthStencilViewVk>,
    ) where
        TIterator: Iterator<Item = ColorTargetViewVk>,
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
        self.attribute_state_infos = Some(vertex_state.clone_attribute_state_infos().to_vec());
        self.buffer_state_infos = Some(vertex_state.clone_buffer_state_infos().to_vec());
    }

    pub fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        self.primitive_topology = Some(primitive_topology);
        self.vertex_count = Some(vertex_count as u32);
        self.vertex_offset = Some(vertex_offset)
    }

    pub fn get_draw_vertex_count(&self) -> u32 {
        *self.vertex_count.as_ref().unwrap()
    }

    pub fn get_draw_vertex_offset(&self) -> i32 {
        *self.vertex_offset.as_ref().unwrap()
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
        if self.compute_shader_module.is_some() {
            self.build_compute_command()
        } else if self.vertex_shader_module.is_some() {
            self.build_graphics_command()
        } else {
            AutoCommandBufferBuilder::primary(
                self.device.clone(),
                self.queue.as_ref().family(),
                vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap()
        }
    }

    fn build_compute_command(&self) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
        let shader = self.compute_shader_module.as_ref().unwrap().clone();
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

        let mut write_descriptor_sets = Vec::new();

        // 定数バッファ
        for index in 0..self.constant_buffers.len() {
            if let Some(buffer) = &self.constant_buffers[index] {
                write_descriptor_sets.push(WriteDescriptorSet::buffer(
                    index as u32,
                    buffer.clone_buffer(),
                ));
            }
        }

        // Unordered Access Buffer
        for index in 0..self.unordered_access_buffer.len() {
            if let Some(buffer) = &self.unordered_access_buffer[index] {
                write_descriptor_sets.push(WriteDescriptorSet::buffer(
                    index as u32,
                    buffer.clone_buffer(),
                ));
            }
        }

        let set =
            PersistentDescriptorSet::new(descriptor_set_layout.clone(), write_descriptor_sets)
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
                set,
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
            .vertex_input_state(BuffersDefinition::new().vertex::<Float32_32>())
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

        let framebuffer = {
            let mut builder = Framebuffer::start(render_pass.clone());

            for view in self.render_targets.as_ref().unwrap() {
                builder = builder.add(view.clone()).unwrap();
            }
            builder.build().unwrap()
        };

        let clear_values = vec![ClearValue::Float(self.clear_color)];
        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [640.0, 480.0],
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
            .draw(self.vertex_count.unwrap() /*vertex buffers*/, 1, 0, 0)
            .unwrap()
            .end_render_pass()
            .unwrap();
        builder
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
        color_target_view: &mut Self::ColorTargetViewType,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        texture_array_range: TextureArrayRange,
    ) {
        self.clear_color(color_target_view, red, green, blue, alpha, texture_array_range);
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
