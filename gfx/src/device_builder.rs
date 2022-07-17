use sjgfx_interface::{DebugMode, DeviceInfo, IDevice};

use crate::api::IApi;

pub struct TDeviceBuilder<T: IApi> {
    info: DeviceInfo,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IApi> TDeviceBuilder<T> {
    pub fn new() -> Self {
        Self {
            info: DeviceInfo::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self) -> T::Device {
        T::Device::new(&DeviceInfo::new())
    }

    pub fn build_with_surface(&self, display: &T::Display) -> T::Device {
        T::Device::new_with_surface(&DeviceInfo::new(), display)
    }

    pub fn enable_debug_assertion(self) -> Self {
        Self {
            info: self.info.set_debug_mode(DebugMode::FullAssertion),
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::{self, IApi},
        TDeviceBuilder,
    };

    #[test]
    fn new() {
        new_impl::<api::Ash>();
        new_impl::<api::Wgpu>();
    }

    fn new_impl<TApi: IApi>() {
        let _device = TDeviceBuilder::<TApi>::new()
            .enable_debug_assertion()
            .build();
    }
}
