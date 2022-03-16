// use raw_window_handle::HasRawWindowHandle;
// use sjgfx_interface::{CommandBufferInfo, DeviceInfo, IDevice, QueueInfo, SwapChainInfo};
// use sjgfx_vulkano::{CommandBufferVk, DeviceVk, QueueVk, SemaphoreVk, ShaderVk, SwapChainVk};
// use winit::event_loop::EventLoop;

// use crate::{
//     CommandBufferBuilder, DeviceBuilder, QueueBuilder, SemaphoreBuilder, ShaderBuilder,
//     SwapChainBuilder,
// };

// pub trait IDeviceBuilderVk {
//     fn build(&self) -> DeviceVk;
//     fn build_widh_surface<TWindow: HasRawWindowHandle>(
//         &self,
//         window: &TWindow,
//         event_loop: &EventLoop<()>,
//     ) -> DeviceVk;
// }

// impl IDeviceBuilderVk for DeviceBuilder {
//     fn build(&self) -> DeviceVk {
//         DeviceVk::new(&DeviceInfo::new())
//     }

//     fn build_widh_surface<TWindow: HasRawWindowHandle>(
//         &self,
//         _window: &TWindow,
//         event_loop: &EventLoop<()>,
//     ) -> DeviceVk {
//         DeviceVk::new_as_graphics(&DeviceInfo::new(), event_loop)
//     }
// }

// pub trait IQueueBuilderVk {
//     fn build(&self, device: &DeviceVk) -> QueueVk;
// }

// impl IQueueBuilderVk for QueueBuilder {
//     fn build(&self, device: &DeviceVk) -> QueueVk {
//         QueueVk::new(device, &QueueInfo::new())
//     }
// }

// pub trait ICommandBufferBuilderVk {
//     fn build<'a>(&self, device: &'a DeviceVk) -> CommandBufferVk<'a>;
// }

// impl ICommandBufferBuilderVk for CommandBufferBuilder {
//     fn build<'a>(&self, device: &'a DeviceVk) -> CommandBufferVk<'a> {
//         CommandBufferVk::new(device, &CommandBufferInfo::new())
//     }
// }

// pub trait ISwapChainBuilderVk {
//     fn build<'a>(&self, device: &DeviceVk) -> SwapChainVk;
// }
// impl ISwapChainBuilderVk for SwapChainBuilder {
//     fn build<'a>(&self, device: &DeviceVk) -> SwapChainVk {
//         SwapChainVk::new(device, &SwapChainInfo::new())
//     }
// }

// pub trait IShaderBuilderVk {
//     fn build(&self, device: &DeviceVk) -> ShaderVk;
// }
// impl IShaderBuilderVk for ShaderBuilder {
//     fn build(&self, device: &DeviceVk) -> ShaderVk {
//         let info = self.create_info();
//         ShaderVk::new(device, &info)
//     }
// }

// pub trait ISemaphoreBuilderVk {
//     fn build(&self, device: &DeviceVk) -> SemaphoreVk;
// }
// impl ISemaphoreBuilderVk for SemaphoreBuilder {
//     fn build(&self, _device: &DeviceVk) -> SemaphoreVk {
//         SemaphoreVk {}
//     }
// }

// #[cfg(test)]
// mod tests {
//     // use sjgfx_interface::{IDevice, DeviceInfo};
//     // use sjgfx_vulkano::DeviceVk;

//     // use crate::*;
//     // use crate::vulkano::IDeviceBuilderVk;
//     // use crate::vulkano::IDeviceBuilderVk;
//     // use crate::vulkano::IQueueBuilderVk;
//     // use crate::{DeviceBuilder, QueueBuilder};

//     #[test]
//     fn temp() {
//         //let _device = DeviceBuilder::new().build();
//         //let _device = DeviceVk::new(&DeviceInfo::new());
//         //let _queue = QueueBuilder::build(&device);
//     }
// }
