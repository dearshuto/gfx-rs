use crate::gfx::ScanBufferCommandBuffer;

use super::super::queue_api::QueueInfo;
use super::super::CommandBuffer;
use super::super::Device;

pub struct QueueImpl<'a> {
    _device: &'a Device,
}

impl<'a> QueueImpl<'a> {
    pub fn get_queue(&self) -> &wgpu::Queue {
        &self._device.to_data().get_queue()
    }
}

impl<'a> super::super::queue_api::IQueueImpl<'a> for QueueImpl<'a> {
    fn new(device: &'a Device, _info: &QueueInfo) -> Self {
        QueueImpl { _device: device }
    }

    fn execute(&mut self, command_buffer: &CommandBuffer<'a>) {
        let device_wgpu = self._device.to_data().get_device();
        let queue_wgpu = self._device.to_data().get_queue();
        let command_buffer_impl = command_buffer.to_data();

        let mut command_encoder =
            device_wgpu.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        for index in 0..command_buffer_impl.get_command_count() as usize {
            if command_buffer_impl.is_graphics_command(0) {
                // 描画パス
                let mut render_pass =
                    command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: command_buffer_impl.get_render_target(0),
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None, //TODO
                    });

                // 描画パイプライン
                render_pass.set_pipeline(command_buffer_impl.get_graphics_pipeline(index));

                // バインドグループ
                render_pass.set_bind_group(0, command_buffer_impl.get_bind_group(index), &[]);

                // インデクスバッファ
                //render_pass.set_index_buffer();

                // 頂点バッファ
                render_pass
                    .set_vertex_buffer(0, command_buffer_impl.get_vertex_buffer(0).slice(..));

                // 描画コマンド
                match command_buffer_impl.get_draw_command(index) {
					super::command_builder::graphics_pass_command_builder::DrawCommand::Draw(_, vertex_count, vertex_offset, instance_count, base_instance) => {
						let vertices_range =  *vertex_offset..(*vertex_offset + *vertex_count);
						let instances_range = *base_instance..(*base_instance + *instance_count);
						render_pass.draw(vertices_range, instances_range);
					},
					super::command_builder::graphics_pass_command_builder::DrawCommand::DrawIndexed(_, _, _, _, _, _, _) => todo!(),
				}
            } else {
                // 演算パス
                let mut compute_pass = command_encoder
                    .begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });

                // パイプライン
                let compute_pipeline = command_buffer_impl.get_compute_pipeline(index as usize);
                compute_pass.set_pipeline(&compute_pipeline);

                // リソース
                let bind_group = command_buffer_impl.get_bind_group(index as usize);
                compute_pass.set_bind_group(0, &bind_group, &[]);

                // ディスパッチ
                let (dispatch_count_x, dispatch_count_y, dispatch_count_z) =
                    command_buffer_impl.get_dispatch_count(index as usize);
                compute_pass.dispatch(dispatch_count_x, dispatch_count_y, dispatch_count_z);
            }
        }

        queue_wgpu.submit(Some(command_encoder.finish()));
    }

    fn execute_scan_buffer_command(&mut self, command_buffer: ScanBufferCommandBuffer) {
        let device_wgpu = self._device.to_data().get_device();
        let queue_wgpu = self._device.to_data().get_queue();
        let command_buffer_wgpu = command_buffer.move_data();
        let view = command_buffer_wgpu.create_texture_view();
        let mut command_encoder =
            device_wgpu.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            // 描画パス
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None, //TODO
            });

            // 描画パイプライン
            render_pass.set_pipeline(command_buffer_wgpu.get_graphics_pipeline());

            // バインドグループ
            render_pass.set_bind_group(0, command_buffer_wgpu.get_bind_group(), &[]);

            // 頂点バッファ
            render_pass.set_vertex_buffer(0, command_buffer_wgpu.get_vertex_buffer().slice(..));

            // 描画コマンド
            match command_buffer_wgpu.get_draw_command() {
                super::command_builder::graphics_pass_command_builder::DrawCommand::Draw(
                    _,
                    vertex_count,
                    vertex_offset,
                    instance_count,
                    base_instance,
                ) => {
                    let vertices_range = *vertex_offset..(*vertex_offset + *vertex_count);
                    let instances_range = *base_instance..(*base_instance + *instance_count);
                    render_pass.draw(vertices_range, instances_range);
                }
                super::command_builder::graphics_pass_command_builder::DrawCommand::DrawIndexed(
                    _primitive_topology,
                    index_format,
                    gpu_address,
                    index_count,
                    base_vertex,
                    instance_count,
                    base_instance,
                ) => {
                    // インデクスバッファ
                    render_pass.set_index_buffer(
                        gpu_address.to_data().get_buffer().get_buffer().slice(..),
                        *index_format,
                    );

                    let instances_range = *base_instance..(*base_instance + *instance_count);
                    render_pass.draw_indexed(0..*index_count, *base_vertex, instances_range);
                }
            }
        }

        queue_wgpu.submit(Some(command_encoder.finish()));
    }

    fn present(&mut self, _swap_chain: &mut crate::gfx::SwapChain, _present_interval: i32) {}

    fn flush(&mut self) {}

    fn sync(&self) {
        self._device
            .to_data()
            .get_device()
            .poll(wgpu::Maintain::Wait);
    }

    fn sync_semaphore(&mut self, _semaphore: &mut crate::gfx::Semaphore) {}
}
