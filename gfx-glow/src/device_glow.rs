use std::sync::Arc;

use glow::HasContext;
use glutin::{dpi::PhysicalSize, window::Window, ContextWrapper, PossiblyCurrent};
use sjgfx_interface::{DeviceInfo, IDevice};
use winit::event_loop::EventLoop;

use crate::vi::Display;

pub struct DeviceGlow {
    context: Arc<glow::Context>,
    window: Option<ContextWrapper<PossiblyCurrent, Window>>,
}

impl DeviceGlow {
    pub fn new_with_display(_info: &DeviceInfo, display: &Display) -> Self {
        let gl =
            unsafe { glow::Context::from_loader_function(|s| display.window.get_proc_address(s)) };

        let error = unsafe { gl.get_error() };
        assert_eq!(error, glow::NO_ERROR);

        Self {
            context: Arc::new(gl),
            window: None,
        }
    }

    pub fn new(_info: &DeviceInfo) -> Self {
        let event_loop = unsafe { &crate::GLOW_STATIC_DATA.as_ref().unwrap().event_loop };
        let window_builder = winit::window::WindowBuilder::new()
            .with_visible(false)
            .with_inner_size(PhysicalSize::new(640, 480));
        let window = unsafe {
            glutin::ContextBuilder::new()
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap()
        };
        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                let _ = 10;
                window.get_proc_address(s)
            })
        };

        let error = unsafe { gl.get_error() };
        assert_eq!(error, glow::NO_ERROR);

        Self {
            context: Arc::new(gl),
            window: Some(window),
        }
    }

    pub fn clone_context(&self) -> Arc<glow::Context> {
        self.context.clone()
    }

    pub fn make_current(&mut self) {
        if self.window.is_some() {
            let mut temp = None;
            std::mem::swap(&mut temp, &mut self.window);
            let new_context = unsafe { temp.unwrap().make_current() }.unwrap();
            self.window = Some(new_context);
        }
    }

    // 仮実装
    pub fn swap_buffers(&mut self) {
        if let Some(window) = &self.window {
            window.swap_buffers().unwrap();
        }
    }
}

impl IDevice for DeviceGlow {
    fn new(_info: &DeviceInfo) -> Self {
        // Self::new(info)
        todo!()
    }

    fn new_with_surface<TWindow>(
        _info: &DeviceInfo,
        _window: &TWindow,
        _event_loop: &EventLoop<()>,
    ) -> Self
    where
        TWindow: raw_window_handle::HasRawWindowHandle,
    {
        // Self::new(info)
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::DeviceInfo;

    use crate::DeviceGlow;

    #[test]
    fn new() {
        // let _device = DeviceGlow::new(&DeviceInfo::new());
    }
}
