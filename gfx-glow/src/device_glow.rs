use std::sync::Arc;

use glow::HasContext;
use glutin::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use sjgfx_interface::{DeviceInfo, IDevice};

pub struct DeviceGlow {
    #[allow(dead_code)]
    event_loop: EventLoop<()>,
    #[allow(dead_code)]
    gl_window: glutin::ContextWrapper<glutin::PossiblyCurrent, Window>,
    context: Arc<glow::Context>,
}

impl DeviceGlow {
    pub fn new(_info: &DeviceInfo) -> Self {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new();

        let gl_window = unsafe {
            glutin::ContextBuilder::new()
                // .with_depth_buffer(native_options.depth_buffer)
                // .with_multisampling(native_options.multisampling)
                // .with_srgb(true)
                // .with_stencil_buffer(native_options.stencil_buffer)
                // .with_vsync(native_options.vsync)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap()
        };

        let gl = unsafe { glow::Context::from_loader_function(|s| gl_window.get_proc_address(s)) };

        let error = unsafe { gl.get_error() };
        assert_eq!(error, glow::NO_ERROR);

        Self {
            event_loop,
            gl_window,
            context: Arc::new(gl),
        }
    }

    pub fn clone_context(&self) -> Arc<glow::Context> {
        self.context.clone()
    }
}

impl IDevice for DeviceGlow {
    fn new(info: &DeviceInfo) -> Self {
        Self::new(info)
    }

    fn new_with_surface<TWindow>(
        info: &DeviceInfo,
        _window: &TWindow,
        _event_loop: &EventLoop<()>,
    ) -> Self
    where
        TWindow: raw_window_handle::HasRawWindowHandle,
    {
        Self::new(info)
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::DeviceInfo;

    use crate::DeviceGlow;

    #[test]
    fn new() {
        let _device = DeviceGlow::new(&DeviceInfo::new());
    }
}
