use sjgfx_interface::{CommandBufferInfo, ICommandBuffer};

use crate::api::IApi;

pub struct TCommandBufferBuilder<T: IApi> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: IApi> TCommandBufferBuilder<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &T::Device) -> T::CommandBuffer {
        T::CommandBuffer::new(device, &CommandBufferInfo::new())
    }
}
