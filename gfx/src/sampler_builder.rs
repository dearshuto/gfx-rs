use sjgfx_interface::{ISampler, SamplerInfo};

use crate::api::IApi;

pub struct TSamplerBuilder<TApi: IApi> {
    info: SamplerInfo,
    _marker: std::marker::PhantomData<TApi>,
}

impl<TApi: IApi> TSamplerBuilder<TApi> {
    pub fn new() -> Self {
        Self {
            info: SamplerInfo::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &mut TApi::Device) -> TApi::Sampler {
        TApi::Sampler::new(device, &self.info)
    }
}
