use sjgfx_interface::{DeviceInfo, IDevice};
use winit::event_loop::EventLoop;

pub struct TDeviceBuilder<T: IDevice> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: IDevice> TDeviceBuilder<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self) -> T {
        T::new(&DeviceInfo::new())
    }

    pub fn build_with_surface<TWindow: raw_window_handle::HasRawWindowHandle>(
        &self,
        window: &TWindow,
        event_loop: &EventLoop<()>,
    ) -> T {
        T::new_with_surface(&DeviceInfo::new(), window, event_loop)
    }
}
