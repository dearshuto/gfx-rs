use sjgfx_interface::{ISemaphore, SemaphoreInfo};

use crate::api::IApi;

pub struct TSemaphoreBuilder<TApi: IApi> {
    _marker: std::marker::PhantomData<TApi>,
}

impl<TApi: IApi> TSemaphoreBuilder<TApi> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &TApi::Device) -> TApi::Semaphore {
        TApi::Semaphore::new(device, &SemaphoreInfo::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::{self, IApi},
        TDeviceBuilder, TSemaphoreBuilder,
    };

    #[test]
    fn new() {
        new_impl::<api::Ash>();
        new_impl::<api::Wgpu>();
    }

    fn new_impl<TApi: IApi>() {
        let device = TDeviceBuilder::<TApi>::new().build();
        let _ = TSemaphoreBuilder::<TApi>::new().build(&device);
    }
}
