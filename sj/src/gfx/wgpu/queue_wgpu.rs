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
        for index in 0..command_buffer_impl.get_command_count() {
            // 演算パス
            let mut compute_pass =
                command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });

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
