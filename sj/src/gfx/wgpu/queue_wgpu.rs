use super::super::queue_api::QueueInfo;
use super::super::Device;
use super::command_buffer_wgpu::CommandBuffer;

pub struct QueueImpl<'a> {
    queue: &'a wgpu::Queue,
}

impl<'a> super::super::queue_api::IQueueImpl<'a> for QueueImpl<'a> {
    fn new(device: &'a mut Device, _info: &QueueInfo) -> Self {
        let queue = device.to_data().get_queue();
        QueueImpl { queue }
    }
}

impl<'a> QueueImpl<'a> {
    pub fn execute(&self, command_buffer: &mut CommandBuffer) {
        let command_buffer = command_buffer.get_command_buffer();
        self.queue.submit(Some(command_buffer));
    }
}
