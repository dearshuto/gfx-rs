use sjgfx_interface::{CommandBufferInfo, ICommandBuffer};

pub struct TCommandBufferBuilder<T: ICommandBuffer> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: ICommandBuffer> TCommandBufferBuilder<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &T::DeviceType) -> T {
        T::new(device, &CommandBufferInfo::new())
    }
}
