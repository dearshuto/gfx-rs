use std::sync::Arc;

use glow::HasContext;
use sjgfx_interface::{ISwapChain, ImageFormat};
use sjvi::IDisplayEventListener;

use crate::{ColorTargetViewGlow, DeviceGlow, FenceGlow, SemaphoerGlow};

pub struct SwapChainGlow {
    gl: Arc<glow::Context>,
    framebuffer: glow::Framebuffer,
    texture: glow::Texture,
    color_target: ColorTargetViewGlow,
}

impl SwapChainGlow {
    pub fn get_framebuffer(&self) -> glow::Framebuffer {
        self.framebuffer
    }

    pub fn get_texture(&self) -> glow::Texture {
        self.texture
    }
}

impl ISwapChain for SwapChainGlow {
    type ColorTargetViewType = ColorTargetViewGlow;
    type DeviceType = DeviceGlow;
    type SemaphoreType = SemaphoerGlow;
    type FenceType = FenceGlow;

    fn new(device: &mut Self::DeviceType, info: &sjgfx_interface::SwapChainInfo) -> Self {
        let gl = device.clone_context();

        // カラーバッファを作成
        let texture = unsafe { gl.create_texture() }.unwrap();
        unsafe { gl.bind_texture(glow::TEXTURE_2D, Some(texture)) }
        unsafe {
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGB as i32,
                info.get_width() as i32,
                info.get_height() as i32,
                0, /*border*/
                glow::RGB,
                glow::UNSIGNED_BYTE,
                None,
            )
        }
        unsafe {
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            )
        }
        unsafe {
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            )
        }
        unsafe {
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            )
        }
        unsafe {
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            )
        }
        unsafe { gl.bind_texture(glow::TEXTURE_2D, None) }

        // カラーターゲットを作成
        let color_target = ColorTargetViewGlow::new_direct(texture, ImageFormat::R8G8B8A8Unorm);

        let framebuffer = unsafe { gl.create_framebuffer() }.unwrap();

        Self {
            gl,
            framebuffer,
            texture,
            color_target,
        }
    }

    fn acquire_next_scan_buffer_view(
        &mut self,
        _semaphore: Option<&mut Self::SemaphoreType>,
        _fence: Option<&mut Self::FenceType>,
    ) -> &mut Self::ColorTargetViewType {
        &mut self.color_target
    }
}

impl Drop for SwapChainGlow {
    fn drop(&mut self) {
        unsafe { self.gl.delete_texture(self.texture) }
        unsafe { self.gl.delete_framebuffer(self.framebuffer) }
    }
}

impl IDisplayEventListener for SwapChainGlow {}
