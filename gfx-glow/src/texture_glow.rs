use std::sync::Arc;

use glow::HasContext;
use sjgfx_interface::ITexture;

use crate::DeviceGlow;

pub struct TextureGlow {
    gl: Arc<glow::Context>,
    texture: glow::Texture,
}

impl TextureGlow {
    pub fn get_handle(&self) -> glow::Texture {
        self.texture
    }

    fn convert_to_target(info: &sjgfx_interface::TextureInfo) -> u32 {
        if info.get_depth() == 1 {
            glow::TEXTURE_2D
        } else if 1 < info.get_depth() {
            glow::TEXTURE_3D
        } else {
            panic!()
        }
    }
}

impl ITexture for TextureGlow {
    type DeviceType = DeviceGlow;

    fn new(device: &mut Self::DeviceType, info: &sjgfx_interface::TextureInfo) -> Self {
        device.make_current();
        let gl = device.clone_context();
        let texture = unsafe { gl.create_texture() }.unwrap();

        let target = Self::convert_to_target(info);
        unsafe { gl.bind_texture(target, Some(texture)) }
        unsafe { gl.bind_texture(target, None) }

        Self { gl, texture }
    }

    fn new_with_data(
        _device: &Self::DeviceType,
        _info: &sjgfx_interface::TextureInfo,
        _data: &[u8],
    ) -> Self {
        todo!()
    }
}

impl Drop for TextureGlow {
    fn drop(&mut self) {
        unsafe { self.gl.delete_texture(self.texture) }
    }
}
