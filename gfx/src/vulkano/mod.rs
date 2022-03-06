use sjgfx_interface::{CommandBufferInfo, DeviceInfo, IDevice, QueueInfo};
use sjgfx_vulkano::{CommandBufferVk, DeviceVk, QueueVk};

use crate::{CommandBufferBuilder, DeviceBuilder, QueueBuilder};

pub trait IDeviceBuilderVk {
    fn build(&self) -> DeviceVk;
}

impl IDeviceBuilderVk for DeviceBuilder {
    fn build(&self) -> DeviceVk {
        DeviceVk::new(&DeviceInfo::new())
    }
}

pub trait IQueueBuilderVk {
    fn build(&self, device: &DeviceVk) -> QueueVk;
}

impl IQueueBuilderVk for QueueBuilder {
    fn build(&self, device: &DeviceVk) -> QueueVk {
        QueueVk::new(device, &QueueInfo::new())
    }
}

pub trait ICommandBufferBuilderVk {
    fn build<'a>(&self, device: &'a DeviceVk) -> CommandBufferVk<'a>;
}

impl ICommandBufferBuilderVk for CommandBufferBuilder {
    fn build<'a>(&self, device: &'a DeviceVk) -> CommandBufferVk<'a> {
        CommandBufferVk::new(device, &CommandBufferInfo::new())
    }
}

#[cfg(test)]
mod tests {
    // use sjgfx_interface::{IDevice, DeviceInfo};
    // use sjgfx_vulkano::DeviceVk;

    // use crate::*;
    // use crate::vulkano::IDeviceBuilderVk;
    // use crate::vulkano::IDeviceBuilderVk;
    // use crate::vulkano::IQueueBuilderVk;
    // use crate::{DeviceBuilder, QueueBuilder};

    #[test]
    fn temp() {
        //let _device = DeviceBuilder::new().build();
        //let _device = DeviceVk::new(&DeviceInfo::new());
        //let _queue = QueueBuilder::build(&device);
    }
}
