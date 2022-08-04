use std::sync::Arc;

use sjgfx_interface::IQueue;
use web_sys::WebGl2RenderingContext;

use crate::{CommandBufferWsys, DeviceWsys, FenceWsys, SwapChainWsys};

pub struct QueueWsys {
    gl: Arc<WebGl2RenderingContext>,
}

impl IQueue for QueueWsys {
    type DeviceType = DeviceWsys;
    type CommandBufferType = CommandBufferWsys;
    type FenceType = FenceWsys;
    type SwapChainType = SwapChainWsys;

    fn new(device: &mut Self::DeviceType, _info: &sjgfx_interface::QueueInfo) -> Self {
        let gl = device.clone_context();

        Self { gl }
    }

    fn execute(&mut self, command_buffer: &Self::CommandBufferType) {
        // シェーダ
        let shader = command_buffer.try_get_shader();
        self.gl.use_program(shader);

        // コマンド
        if let Some(command) = command_buffer.try_get_command() {
            match command {
                crate::command_buffer_wsys::Command::Draw(ref info) => {
                    self.gl
                        .draw_arrays(info.primitive_topology, info.offset, info.vertex_count)
                }
                crate::command_buffer_wsys::Command::Dispatch(_) => todo!(),
            }
        }
    }

    fn execute_with_fence(
        &mut self,
        command_buffer: &Self::CommandBufferType,
        _fence: &mut Self::FenceType,
    ) {
        self.execute(command_buffer)
    }

    fn present(&mut self, _swap_chain: &mut Self::SwapChainType) {

    }

    fn flush(&mut self) {
        self.gl.flush()
    }

    fn sync(&mut self) {}
}
