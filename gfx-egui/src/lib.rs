mod platform_glow;
mod platform_winit;
use platform_glow::PlatformGlow;
use platform_winit::PlatformWinit;

use sjgfx::{
    api::IApi, TBufferBuilder, TCommandBufferBuilder, TSamplerBuilder, TShaderBuilder,
    TTextureBuilder, TTextureViewBuilder, TVertexStateBuilder,
};
use sjgfx_interface::{
    AttributeFormat, IBuffer, ICommandBuffer, IQueue, ITextureView, ImageFormat, IndexFormat,
    PrimitiveTopology, ScissorStateInfo, TextureViewInfo, VertexAttributeStateInfo,
    VertexBufferStateInfo,
};

pub struct RenderPass<TApi: IApi> {
    command_buffer: TApi::CommandBuffer,
    shader: TApi::Shader,
    constant_buffer: BufferCache<TApi>,
    index_buffers: Vec<BufferCache<TApi>>,
    vertex_buffers: Vec<BufferCache<TApi>>,
    vertex_state: TApi::VertexState,
    texture: TextureCache<TApi>,
    sampler: TApi::Sampler,
}

impl<TApi: IApi> RenderPass<TApi> {
    pub fn new(device: &mut TApi::Device) -> Self {
        let mut shader_compiler = sjgfx_util::ShaderCompiler::new();
        let vertex_shader_binary = shader_compiler.create_binary(
            include_str!("../resources/egui.vs"),
            sjgfx_util::ShaderStage::Vertex,
        );
        let pixel_shader_binary = shader_compiler.create_binary(
            include_str!("../resources/egui.fs"),
            sjgfx_util::ShaderStage::Pixel,
        );
        let shader = TShaderBuilder::<TApi>::new()
            .set_vertex_shader_binary(&vertex_shader_binary)
            .set_pixel_shader_binary(&pixel_shader_binary)
            .build(device);

        let command_buffer = TCommandBufferBuilder::<TApi>::new().build(device);

        let texture = TTextureBuilder::<TApi>::new()
            .enable_sampler()
            .with_format(ImageFormat::R8G8B8A8Unorm)
            .with_size(1280, 960)
            .build(device);
        let texture_view = TTextureViewBuilder::<TApi>::new()
            .with_format(ImageFormat::R8G8B8A8Unorm)
            .build(device, &texture);

        let sampler = TSamplerBuilder::<TApi>::new().build(device);

        let constant_buffer = TBufferBuilder::<TApi>::new()
            .enable_constant_buffer()
            .with_size(16)
            .build(device);

        let vertex_state = TVertexStateBuilder::<TApi>::new()
            .set_vertex_attribute_states(
                [
                    VertexAttributeStateInfo::new()
                        .set_buffer_index(0)
                        .set_format(AttributeFormat::Float32_32)
                        .set_offset(0)
                        .set_slot(0),
                    VertexAttributeStateInfo::new()
                        .set_buffer_index(0)
                        .set_format(AttributeFormat::Float32_32)
                        .set_offset((std::mem::size_of::<f32>() * 2) as i64)
                        .set_slot(1),
                    VertexAttributeStateInfo::new()
                        .set_buffer_index(0)
                        .set_format(AttributeFormat::Uint32)
                        .set_offset((std::mem::size_of::<f32>() * 4) as i64)
                        .set_slot(2),
                ]
                .into_iter(),
            )
            .set_vertex_buffer_states(
                [VertexBufferStateInfo::new().set_stride(
                    (std::mem::size_of::<f32>() * 2
                        + std::mem::size_of::<f32>() * 2
                        + std::mem::size_of::<u32>()) as i64,
                )]
                .into_iter(),
            )
            .build(device);

        Self {
            command_buffer,
            shader,
            index_buffers: Vec::new(),
            vertex_buffers: Vec::new(),
            texture: TextureCache {
                version: 0,
                texture_view,
                texture,
            },
            sampler,
            constant_buffer: BufferCache {
                size: 64,
                buffer: constant_buffer,
            },
            vertex_state,
        }
    }

    pub fn update_texture(&mut self, device: &mut TApi::Device, font_image: &egui::FontImage) {
        if self.texture.version == font_image.version {
            return;
        } else {
            let mut data = Vec::with_capacity(font_image.pixels.len() * 4 /*RGBA*/);
            for rgba in font_image.srgba_pixels(1.0) {
                data.push(rgba.r());
                data.push(rgba.g());
                data.push(rgba.b());
                data.push(rgba.a());
            }

            let texture = TTextureBuilder::<TApi>::new()
                .enable_sampler()
                .with_size(font_image.width as i32, font_image.height as i32)
                .with_format(ImageFormat::R8G8B8A8Unorm)
                .with_data(&data)
                .build(device);

            let texture_view_info = TextureViewInfo::new().set_format(ImageFormat::R8G8B8A8Unorm);
            let texture_view = TApi::TextureView::new(device, &texture_view_info, &texture);

            self.texture = TextureCache {
                version: font_image.version,
                texture: texture,
                texture_view: texture_view,
            };
        }
    }

    pub fn update_user_textures(&mut self) {}

    pub fn update_buffers(
        &mut self,
        device: &mut TApi::Device,
        paint_jobs: &[egui::epaint::ClippedMesh],
    ) {
        for (index, egui::ClippedMesh(_, mesh)) in paint_jobs.iter().enumerate() {
            // 定数バッファの更新
            let data = bytemuck::cast_slice(&[1280 as f32, 960 as f32, 0.0, 0.0]);
            if let Some(buffer_cache) = Self::update_buffer(device, &mut self.constant_buffer, data)
            {
                self.constant_buffer = buffer_cache;
            }

            // インデックスバッファの更新
            let data: &[u8] = bytemuck::cast_slice(&mesh.indices);
            if index < self.index_buffers.len() {
                if let Some(buffer_cache) =
                    Self::update_buffer(device, &mut self.index_buffers[index], data)
                {
                    self.index_buffers[index] = buffer_cache;
                }
            } else {
                let buffer = TBufferBuilder::<TApi>::new()
                    .enable_index_buffer()
                    .with_size(data.len())
                    .build(device);
                self.index_buffers.push(BufferCache {
                    size: data.len(),
                    buffer,
                });
            }

            // 頂点バッファの更新
            let data: &[u8] = bytemuck::cast_slice(&mesh.vertices);
            if index < self.vertex_buffers.len() {
                if let Some(buffer_cache) =
                    Self::update_buffer(device, &mut self.vertex_buffers[index], data)
                {
                    self.vertex_buffers[index] = buffer_cache;
                }
            } else {
                let buffer = TBufferBuilder::<TApi>::new()
                    .enable_vertex_buffer()
                    .with_size(data.len())
                    .build(device);
                self.vertex_buffers.push(BufferCache {
                    size: data.len(),
                    buffer,
                });
            }
        }
    }

    pub fn execute(
        &mut self,
        color_target_view: &TApi::ColorTargetView,
        queue: &mut TApi::Queue,
        paint_jobs: &[egui::epaint::ClippedMesh],
    ) {
        let physical_width = 2560; //screen_descriptor.physical_width;
        let physical_height = 1920; //screen_descriptor.physical_height;
        let scale_factor = 1.0;

        for ((egui::ClippedMesh(clip_rect, mesh), vertex_buffer), index_buffer) in paint_jobs
            .iter()
            .zip(self.vertex_buffers.iter())
            .zip(self.index_buffers.iter())
        {
            let _scissor_state_info = {
                // Transform clip rect to physical pixels.
                let clip_min_x = scale_factor * clip_rect.min.x;
                let clip_min_y = scale_factor * clip_rect.min.y;
                let clip_max_x = scale_factor * clip_rect.max.x;
                let clip_max_y = scale_factor * clip_rect.max.y;

                // Make sure clip rect can fit within an `u32`.
                let clip_min_x = clip_min_x.clamp(0.0, physical_width as f32);
                let clip_min_y = clip_min_y.clamp(0.0, physical_height as f32);
                let clip_max_x = clip_max_x.clamp(clip_min_x, physical_width as f32);
                let clip_max_y = clip_max_y.clamp(clip_min_y, physical_height as f32);

                let clip_min_x = clip_min_x.round() as u32;
                let clip_min_y = clip_min_y.round() as u32;
                let clip_max_x = clip_max_x.round() as u32;
                let clip_max_y = clip_max_y.round() as u32;

                let width = (clip_max_x - clip_min_x).max(1);
                let height = (clip_max_y - clip_min_y).max(1);

                // Clip scissor rectangle to target size.
                let x = clip_min_x.min(physical_width);
                let y = clip_min_y.min(physical_height);
                let width = width.min(physical_width - x);
                let height = height.min(physical_height - y);

                // Skip rendering with zero-sized clip areas.
                if width == 0 || height == 0 {
                    continue;
                }

                // rpass.set_scissor_rect(x, y, width, height);
                ScissorStateInfo::new()
                    .set_origin_x(x as i32)
                    .set_origin_y(y as i32)
                    .set_width(width as i32)
                    .set_height(height as i32)
            };

            self.command_buffer.begin();
            self.command_buffer
                .set_render_targets(&[&color_target_view], None);
            self.command_buffer.set_shader(&self.shader);
            // self.command_buffer.set_scissor(&scissor_state_info);
            self.command_buffer
                .set_texture(0, &self.texture.texture_view);
            self.command_buffer.set_sampler(1, &self.sampler);
            self.command_buffer
                .set_constant_buffer(2, &self.constant_buffer.buffer);
            self.command_buffer
                .set_vertex_buffer(0, &vertex_buffer.buffer);
            self.command_buffer.set_vertex_state(&self.vertex_state);
            self.command_buffer.draw_indexed(
                PrimitiveTopology::TriangleList,
                IndexFormat::Uint32,
                &index_buffer.buffer,
                mesh.indices.len() as i32,
                0,
            );
            self.command_buffer.end();

            queue.execute(&self.command_buffer);
        }
    }

    fn update_buffer(
        device: &mut TApi::Device,
        buffer_cache: &mut BufferCache<TApi>,
        data: &[u8],
    ) -> Option<BufferCache<TApi>> {
        if data.len() > buffer_cache.size {
            // バッファのサイズが足りないのでインスタンスを作り直す
            let buffer = TBufferBuilder::<TApi>::new()
                .enable_constant_buffer()
                .enable_index_buffer()
                .enable_vertex_buffer()
                .with_size(data.len())
                .build(device);

            // データを更新
            buffer.map_as_slice_mut(|x: &mut [u8]| {
                x.clone_from_slice(data);
            });
            buffer.flush_mapped_range(0, data.len());

            Some(BufferCache {
                size: data.len(),
                buffer,
            })
        } else {
            // データを更新
            let buffer = &mut buffer_cache.buffer;
            buffer.map_as_slice_mut(|x: &mut [u8]| {
                let t = &mut x[0..data.len()];
                t.clone_from_slice(data);
            });
            buffer.flush_mapped_range(0, data.len());

            // インスタンスは使いまわすので None を返す
            None
        }
    }
}

struct BufferCache<TApi: IApi> {
    pub size: usize,
    pub buffer: TApi::Buffer,
}

struct TextureCache<TApi: IApi> {
    pub version: u64,
    pub texture_view: TApi::TextureView,
    #[allow(dead_code)]
    texture: TApi::Texture,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
