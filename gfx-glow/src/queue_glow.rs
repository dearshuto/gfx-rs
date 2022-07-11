use std::sync::Arc;

use glow::HasContext;
use sjgfx_interface::QueueInfo;

use crate::{CommandBufferGlow, DeviceGlow, DrawCommand};

pub struct QueueGlow {
    gl: Arc<glow::Context>,
    vertex_array_object: Option<glow::VertexArray>,
}

impl QueueGlow {
    pub fn new(device: &DeviceGlow, _info: &QueueInfo) -> Self {
        Self {
            gl: device.clone_context(),
            vertex_array_object: None,
        }
    }

    pub fn execute(&mut self, command_buffer: &CommandBufferGlow) {
        // 画面のクリア
        unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT) }
        unsafe { self.gl.clear_color(0.2, 0.3, 0.35, 0.0) }

        // シェーダ
        unsafe { self.gl.use_program(command_buffer.try_get_program()) }

        // VAO
        let vao = command_buffer.try_get_vertex_array_object();
        unsafe { self.gl.bind_vertex_array(vao) }

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
        if let Some(vao) = self.vertex_array_object {
            unsafe { self.gl.delete_vertex_array(vao) }
        }
    }
}
