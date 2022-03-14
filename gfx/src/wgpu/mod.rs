// use raw_window_handle::HasRawWindowHandle;
// use sjgfx_interface::{CommandBufferInfo, DeviceInfo, IDevice, QueueInfo, SwapChainInfo};
// use sjgfx_wgpu::{
//     CommandBufferWgpu, DeviceWgpu, FenceWgpu, QueueWgpu, SemaphoreWgpu, ShaderWgpu, SwapChainWgpu,
// };
// use winit::event_loop::EventLoop;

// use crate::{
//     CommandBufferBuilder, DeviceBuilder, FenceBuilder, QueueBuilder, SemaphoreBuilder,
//     ShaderBuilder, SwapChainBuilder,
// };

// pub trait IDeviceBuilderWgpu {
//     fn build(&self) -> DeviceWgpu {
//         DeviceWgpu::new(&DeviceInfo::new())
//     }
//     fn build_widh_surface<TWindow: HasRawWindowHandle>(
//         &self,
//         window: &TWindow,
//         event_loop: &EventLoop<()>,
//     ) -> DeviceWgpu;
// }

// impl IDeviceBuilderWgpu for DeviceBuilder {
//     fn build(&self) -> DeviceWgpu {
//         DeviceWgpu::new(&DeviceInfo::new())
//     }

//     fn build_widh_surface<TWindow: HasRawWindowHandle>(
//         &self,
//         window: &TWindow,
//         _event_loop: &EventLoop<()>,
//     ) -> DeviceWgpu {
//         DeviceWgpu::new_as_graphics(&DeviceInfo::new(), window)
//     }
// }

// pub trait IQueueBuilderWgpu {
//     fn build<'a>(&self, device: &'a DeviceWgpu) -> QueueWgpu<'a>;
// }

// impl IQueueBuilderWgpu for QueueBuilder {
//     fn build<'a>(&self, device: &'a DeviceWgpu) -> QueueWgpu<'a> {
//         QueueWgpu::new(&device, &QueueInfo::new())
//     }
// }

// pub trait ICommandBufferBuilderWgpu {
//     fn build<'a>(&self, device: &'a DeviceWgpu) -> CommandBufferWgpu<'a>;
// }

// impl ICommandBufferBuilderWgpu for CommandBufferBuilder {
//     fn build<'a>(&self, device: &'a DeviceWgpu) -> CommandBufferWgpu<'a> {
//         CommandBufferWgpu::new(&device, &CommandBufferInfo::new())
//     }
// }

// pub trait ISwapChainBuilderWgpu {
//     fn build<'a>(&self, device: &'a DeviceWgpu) -> SwapChainWgpu<'a>;
// }
// impl ISwapChainBuilderWgpu for SwapChainBuilder {
//     fn build<'a>(&self, device: &'a DeviceWgpu) -> SwapChainWgpu<'a> {
//         SwapChainWgpu::new(device, &SwapChainInfo::new())
//     }
// }

// pub trait IShaderBuilderWgpu {
//     fn build(&self, device: &DeviceWgpu) -> ShaderWgpu;
// }
// impl IShaderBuilderWgpu for ShaderBuilder {
//     fn build(&self, device: &DeviceWgpu) -> ShaderWgpu {
//         let info = self.create_info();
//         ShaderWgpu::new(device, &info)
//     }
// }

// pub trait IFenceBuilderWgpu {
//     fn build(&self, device: &DeviceWgpu) -> FenceWgpu;
// }
// impl IFenceBuilderWgpu for FenceBuilder {
//     fn build(&self, _device: &DeviceWgpu) -> FenceWgpu {
//         FenceWgpu {}
//     }
// }

// pub trait ISemaphoreBuilderWgpu {
//     fn build(&self, device: &DeviceWgpu) -> SemaphoreWgpu;
// }
// impl ISemaphoreBuilderWgpu for SemaphoreBuilder {
//     fn build(&self, _device: &DeviceWgpu) -> SemaphoreWgpu {
//         SemaphoreWgpu {}
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::wgpu::IDeviceBuilderWgpu;
//     use crate::wgpu::IQueueBuilderWgpu;
//     use crate::{DeviceBuilder, QueueBuilder};

//     #[test]
//     fn new() {
//         let device = DeviceBuilder::new().build();
//         let _queue = QueueBuilder::new().build(&device);
//     }
// }
