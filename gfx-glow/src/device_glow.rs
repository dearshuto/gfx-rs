use std::sync::Arc;

use crate::vi::Display;
use glow::HasContext;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder},
    display::DisplayApiPreference,
    prelude::{GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor},
};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use sjgfx_interface::{DeviceInfo, IDevice};

pub struct DeviceGlow {
    context: Arc<glow::Context>,
}

impl DeviceGlow {
    pub fn new_with_handle<THandle>(_info: &DeviceInfo, raw_handle: &THandle) -> Self
    where
        THandle: HasRawDisplayHandle + HasRawWindowHandle,
    {
        let preference = DisplayApiPreference::Cgl;
        let display =
            unsafe { glutin::display::Display::new(raw_handle.raw_display_handle(), preference) }
                .unwrap();
        let template = ConfigTemplateBuilder::new()
            .compatible_with_native_window(raw_handle.raw_window_handle())
            .build();

        let configs = unsafe { display.find_configs(template) }.unwrap();
        let gl_config = configs
            .reduce(|accum, config| {
                let transparency_check = config.supports_transparency().unwrap_or(false)
                    & !accum.supports_transparency().unwrap_or(false);

                if transparency_check || config.num_samples() > accum.num_samples() {
                    config
                } else {
                    accum
                }
            })
            .unwrap();

        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_handle.raw_window_handle()));
        let context_attributes =
            ContextAttributesBuilder::new().build(Some(raw_handle.raw_window_handle()));
        let gl_context =
            unsafe { display.create_context(&gl_config, &context_attributes) }.unwrap();

        todo!()
    }

    pub fn new_with_display(_info: &DeviceInfo, display: &Display) -> Self {
        let gl =
            unsafe { glow::Context::from_loader_function(|s| display.window.get_proc_address(s)) };

        let error = unsafe { gl.get_error() };
        assert_eq!(error, glow::NO_ERROR);

        Self {
            context: Arc::new(gl),
        }
    }

    pub fn new(_info: &DeviceInfo) -> Self {
        todo!()
        // let event_loop = unsafe { &crate::GLOW_STATIC_DATA.as_ref().unwrap().event_loop };
        // let window_builder = WindowBuilder::new()
        //     .with_visible(false)
        //     .with_inner_size(PhysicalSize::new(640, 480));
        // let window = unsafe {
        //     glutin::ContextBuilder::new()
        //         .build_windowed(window_builder, &event_loop)
        //         .unwrap()
        //         .make_current()
        //         .unwrap()
        // };
        // let gl = unsafe {
        //     glow::Context::from_loader_function(|s| {
        //         let _ = 10;
        //         window.get_proc_address(s)
        //     })
        // };

        // let error = unsafe { gl.get_error() };
        // assert_eq!(error, glow::NO_ERROR);

        // Self {
        //     context: Arc::new(gl),
        //     window: Some(window),
        // }
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

    fn new_with_handle<T>(_info: &DeviceInfo, _raw_handle: &T) -> Self
    where
        T: HasRawWindowHandle + HasRawDisplayHandle,
    {
        todo!()
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
