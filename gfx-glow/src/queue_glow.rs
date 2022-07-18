use std::{mem::size_of, sync::Arc};

use glow::HasContext;
use sjgfx_interface::{IQueue, QueueInfo, ShaderInfo};

use crate::{CommandBufferGlow, DeviceGlow, DrawCommand, FenceGlow, ShaderGlow, SwapChainGlow};

pub struct QueueGlow {
    gl: Arc<glow::Context>,

    // スキャンバッファのコピー用オブジェクトたち
    vertex_array_object: glow::VertexArray,
    shader: ShaderGlow,
    vertex_buffer: glow::Buffer,
}

impl QueueGlow {
    pub fn execute(&mut self, command_buffer: &CommandBufferGlow) {
        let error = unsafe { self.gl.get_error() };
        if error != glow::NO_ERROR {
            println!("FBO ERROR: {}", error);
        }

        unsafe {
            self.gl.bind_framebuffer(
                glow::FRAMEBUFFER,
                command_buffer.try_get_frame_buffer_object(),
            )
        }
        let error = unsafe { self.gl.get_error() };
        if error != glow::NO_ERROR {
            println!("EXECUTE BIND FBO ERROR: {}", error);
        }

        // 画面のクリア
        unsafe { self.gl.clear_color(0.2, 0.3, 0.35, 0.0) }
        unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT) }

        // シェーダ
        unsafe { self.gl.use_program(command_buffer.try_get_program()) }

        // VAO
        let vao = command_buffer.try_get_vertex_array_object();
        unsafe { self.gl.bind_vertex_array(vao) }

        // 定数バッファ
        for index in 0..command_buffer.get_constant_buffers().len() {
            let constant_buffer = command_buffer.get_constant_buffers()[index];
            unsafe {
                self.gl
                    .bind_buffer_base(glow::UNIFORM_BUFFER, index as u32, constant_buffer)
            }
        }

        // コマンド
        if let Some(command) = command_buffer.try_get_command() {
            match command {
                DrawCommand::Draw(ref info) => unsafe {
                    self.gl.draw_arrays(
                        info.primitive_topology,
                        info.vertex_offset,
                        info.vertex_count,
                    )
                },
                DrawCommand::DrawInstanced(ref info) => unsafe {
                    self.gl.draw_arrays_instanced_base_instance(
                        info.primitive_topology,
                        info.vertex_offset,
                        info.vertex_count,
                        info.instance_count,
                        info.base_instance,
                    );
                },
                DrawCommand::DrawIndexed(ref info) => unsafe {
                    self.gl
                        .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(info.buffer));
                    self.gl.draw_elements_base_vertex(
                        info.primitive_topology,
                        info.index_count,
                        info.mode,
                        0,
                        info.base_vertex,
                    );
                },
                DrawCommand::DrawIndexedInstanced(ref info) => unsafe {
                    self.gl
                        .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(info.buffer));
                    self.gl.draw_elements_instanced_base_vertex_base_instance(
                        info.primitive_topology,
                        info.index_count,
                        info.mode,
                        0,
                        info.base_vertex,
                        info.instance_count,
                        info.base_instance,
                    );
                },
                DrawCommand::Dispatch(ref info) => unsafe {
                    self.gl
                        .dispatch_compute(info.count_x, info.count_y, info.count_z);
                },
            }
        }
    }
}

impl Drop for QueueGlow {
    fn drop(&mut self) {
        unsafe { self.gl.delete_buffer(self.vertex_buffer) }
    }
}

impl IQueue for QueueGlow {
    type DeviceType = DeviceGlow;
    type CommandBufferType = CommandBufferGlow;
    type FenceType = FenceGlow;
    type SwapChainType = SwapChainGlow;

    fn new(device: &mut Self::DeviceType, _info: &QueueInfo) -> Self {
        let gl = device.clone_context();
        let shader = ShaderGlow::new(
            device,
            &ShaderInfo::new()
                .set_vertex_shader_source(include_str!(
                    "../resources/shaders/frame_buffer_renderer.vs"
                ))
                .set_pixel_shader_source(include_str!(
                    "../resources/shaders/frame_buffer_renderer.fs"
                )),
        );

        let vertex_array_object = unsafe { gl.create_vertex_array() }.unwrap();
        unsafe { gl.bind_vertex_array(Some(vertex_array_object)) }

        let vertex_buffer = unsafe { gl.create_buffer() }.unwrap();
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
        }
        unsafe {
            gl.buffer_data_size(
                glow::ARRAY_BUFFER,
                (size_of::<VertexData>() * 6) as i32,
                glow::STATIC_DRAW,
            );
        }

        // 頂点情報
        let mapped_data = unsafe {
            gl.map_buffer_range(
                glow::ARRAY_BUFFER,
                0,                                    /*offset*/
                (size_of::<VertexData>() * 6) as i32, /*length*/
                glow::MAP_WRITE_BIT,
            )
        };
        let vertex_data: &mut [VertexData] =
            unsafe { std::slice::from_raw_parts_mut(mapped_data as *mut VertexData, 6) };
        vertex_data[0] = VertexData {
            x: -1.0,
            y: 1.0,
            u: 0.0,
            v: 1.0,
        };
        vertex_data[1] = VertexData {
            x: -1.0,
            y: -1.0,
            u: 0.0,
            v: 0.0,
        };
        vertex_data[2] = VertexData {
            x: 1.0,
            y: -1.0,
            u: 1.0,
            v: 0.0,
        };
        vertex_data[3] = VertexData {
            x: -1.0,
            y: 1.0,
            u: 0.0,
            v: 1.0,
        };
        vertex_data[4] = VertexData {
            x: 1.0,
            y: -1.0,
            u: 1.0,
            v: 0.0,
        };
        vertex_data[5] = VertexData {
            x: 1.0,
            y: 1.0,
            u: 1.0,
            v: 1.0,
        };
        unsafe { gl.unmap_buffer(glow::ARRAY_BUFFER) }

        // 頂点座標
        unsafe { gl.enable_vertex_attrib_array(0) }
        unsafe {
            gl.vertex_attrib_pointer_f32(
                0, /*slot*/
                2, /*count*/
                glow::FLOAT,
                false,
                4 * std::mem::size_of::<f32>() as i32,
                0, /*offset*/
            )
        }
        // UV
        unsafe { gl.enable_vertex_attrib_array(1) }
        unsafe {
            gl.vertex_attrib_pointer_f32(
                1, /*slot*/
                2, /*count*/
                glow::FLOAT,
                false,
                4 * std::mem::size_of::<f32>() as i32,
                2 * size_of::<f32>() as i32, /*offset*/
            )
        }
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }
        unsafe { gl.bind_vertex_array(None) }

        Self {
            gl,
            vertex_array_object,
            shader,
            vertex_buffer,
        }
    }

    fn execute(&mut self, command_buffer: &Self::CommandBufferType) {
        self.execute(command_buffer)
    }

    fn execute_with_fence(
        &mut self,
        command_buffer: &Self::CommandBufferType,
        _fence: &mut Self::FenceType,
    ) {
        self.execute(command_buffer)
    }

    fn present(&mut self, swap_chain: &mut Self::SwapChainType) {
        // スキャンバッファをレンダーターゲットに設定
        unsafe { self.gl.bind_framebuffer(glow::FRAMEBUFFER, None) }
        let error = unsafe { self.gl.get_error() };
        if error != glow::NO_ERROR {
            println!("BIND FBO ERROR: {}", error);
        }

        //
        // クリアはしない。どうせ画面全体に描画される
        //

        // シェーダ
        unsafe { self.gl.use_program(Some(self.shader.get_program())) }

        // フレームバッファテクスチャをバインド
        let framebuffer_texture = swap_chain.get_texture();
        unsafe {
            self.gl
                .bind_texture(glow::TEXTURE_2D, Some(framebuffer_texture));

            // let location = self.gl.get_uniform_location(self.shader.get_program(), &"u_FrameBuffer");
            // self.gl.uniform_1_i32(location.as_ref(), 0);
        }
        let error = unsafe { self.gl.get_error() };
        if error != glow::NO_ERROR {
            println!("BIND TEXTURE ERROR: {}", error);
        }

        // バッファ
        unsafe { self.gl.bind_vertex_array(Some(self.vertex_array_object)) }
        let error = unsafe { self.gl.get_error() };
        if error != glow::NO_ERROR {
            println!("BIND BUFFER ERROR: {}", error);
        }

        // 四角形を描画
        unsafe { self.gl.draw_arrays(glow::TRIANGLES, 0, 6) }
    }

    fn flush(&mut self) {
        unsafe { self.gl.flush() }
    }

    fn sync(&mut self) {}
}

#[repr(C)]
struct VertexData {
    x: f32,
    y: f32,
    u: f32,
    v: f32,
}
