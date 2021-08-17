#[cfg(test)]
mod tests {
    use super::super::super::gfx::{CommandBuffer, CommandBufferInfo, Device, DeviceInfo};

    #[test]
    fn initialize() {
        let device = Device::new(&DeviceInfo::new());
        let mut command_buffer = CommandBuffer::new(&device, &CommandBufferInfo::new());
        command_buffer.begin();
        command_buffer.end();
    }
}
