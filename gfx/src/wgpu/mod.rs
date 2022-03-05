use sjgfx_interface::{CommandBufferInfo, DeviceInfo, IDevice, QueueInfo};
use sjgfx_wgpu::{CommandBufferWgpu, DeviceWgpu, QueueWgpu};

use crate::{CommandBufferBuilder, DeviceBuilder, QueueBuilder};

pub trait IDeviceBuilderWgpu {
    fn build(&self) -> DeviceWgpu {
        DeviceWgpu::new(&DeviceInfo::new())
    }
}

impl IDeviceBuilderWgpu for DeviceBuilder {
    fn build(&self) -> DeviceWgpu {
        DeviceWgpu::new(&DeviceInfo::new())
    }
}

pub trait IQueueBuilderWgpu {
    fn build<'a>(&self, device: &'a DeviceWgpu) -> QueueWgpu<'a>;
}

impl IQueueBuilderWgpu for QueueBuilder {
    fn build<'a>(&self, device: &'a DeviceWgpu) -> QueueWgpu<'a> {
        QueueWgpu::new(&device, &QueueInfo::new())
    }
}

pub trait ICommandBufferBuilderWgpu {
    fn build<'a>(&self, device: &'a DeviceWgpu) -> CommandBufferWgpu<'a>;
}

impl ICommandBufferBuilderWgpu for CommandBufferBuilder {
    fn build<'a>(&self, device: &'a DeviceWgpu) -> CommandBufferWgpu<'a> {
        CommandBufferWgpu::new(&device, &CommandBufferInfo::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::wgpu::IDeviceBuilderWgpu;
    use crate::wgpu::IQueueBuilderWgpu;
    use crate::{DeviceBuilder, QueueBuilder};

    #[test]
    fn new() {
        let device = DeviceBuilder::new().build();
        let _queue = QueueBuilder::new().build(&device);
    }
}
