use sjgfx_interface::{DeviceInfo, IDevice};
use winit::event_loop::EventLoop;

use crate::api::IApi;

pub struct TDeviceBuilder<T: IApi> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: IApi> TDeviceBuilder<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self) -> T::Device {
        T::Device::new(&DeviceInfo::new())
    }

    pub fn build_with_surface<TWindow: raw_window_handle::HasRawWindowHandle>(
        &self,
        window: &TWindow,
        event_loop: &EventLoop<()>,
    ) -> T::Device {
        T::Device::new_with_surface(&DeviceInfo::new(), window, event_loop)
    }
}
