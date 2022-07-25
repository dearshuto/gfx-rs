use std::sync::Arc;

use sjgfx_interface::ITexture;
use web_sys::{WebGl2RenderingContext, WebGlTexture};
use web_sys::WebGl2RenderingContext as GL;

use crate::DeviceWsys;

pub struct TextureWsys {
    gl: Arc<WebGl2RenderingContext>,
    texture: WebGlTexture,
}

impl TextureWsys {
    pub fn clone_texture(&self) -> WebGlTexture {
        self.texture.clone()
    }
}

impl ITexture for TextureWsys {
    type DeviceType = DeviceWsys;

    fn new(device: &mut Self::DeviceType, _info: &sjgfx_interface::TextureInfo) -> Self {
        let gl = device.clone_context();
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
        gl.bind_texture(GL::TEXTURE_2D, None);

        Self {
            gl,
            texture,
        }
    }

    fn new_with_data(
        _device: &Self::DeviceType,
        _info: &sjgfx_interface::TextureInfo,
        _data: &[u8],
    ) -> Self {
        todo!()
    }
}

impl Drop for TextureWsys {
    fn drop(&mut self) {
        self.gl.delete_texture(Some(&self.texture));
    }
}
