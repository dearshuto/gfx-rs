use std::sync::Arc;

use sjgfx_interface::{IShader, ShaderInfo};
use vulkano::shader::ShaderModule;

use crate::DeviceVk;

pub struct ShaderVk {
    compute_shader: Option<Arc<ShaderModule>>,
    vertex_shader: Option<Arc<ShaderModule>>,
    pixel_shader: Option<Arc<ShaderModule>>,
}

impl ShaderVk {
    pub fn new(device: &DeviceVk, info: &ShaderInfo) -> Self {
        // 演算シェーダ
        let compute_shader =
            info.get_compute_shader_binary()
                .as_ref()
                .map(|compute_shader_binary| unsafe {
                    ShaderModule::from_bytes(device.clone_device(), compute_shader_binary).unwrap()
                });

        // 頂点シェーダ
        let vertex_shader =
            info.get_vertex_shader_binary()
                .as_ref()
                .map(|vertex_shader_binary| unsafe {
                    ShaderModule::from_bytes(device.clone_device(), vertex_shader_binary).unwrap()
                });

        // ピクセルシェーダ
        let pixel_shader =
            info.get_pixel_shader_binary()
                .as_ref()
                .map(|pixel_shader_binary| unsafe {
                    ShaderModule::from_bytes(device.clone_device(), pixel_shader_binary).unwrap()
                });

        Self {
            compute_shader,
            vertex_shader,
            pixel_shader,
        }
    }

    pub fn is_compute(&self) -> bool {
        self.compute_shader.is_some()
    }

    pub fn get_shader(&self) -> &ShaderModule {
        self.get_compute_shader()
    }

    pub fn clone_shader(&self) -> Arc<ShaderModule> {
        self.clone_compute_shader()
    }

    pub fn get_compute_shader(&self) -> &ShaderModule {
        self.compute_shader.as_ref().unwrap()
    }

    pub fn acquire_compute_shader_module(&self) -> Option<Arc<ShaderModule>> {
        self.compute_shader.clone()
    }

    pub fn clone_compute_shader(&self) -> Arc<ShaderModule> {
        self.compute_shader.as_ref().unwrap().clone()
    }

    pub fn acquire_vertex_shader_module(&self) -> Option<Arc<ShaderModule>> {
        self.vertex_shader.clone()
    }

    pub fn get_vertex_shader(&self) -> &ShaderModule {
        self.vertex_shader.as_ref().unwrap()
    }

    pub fn clone_vertex_shader(&self) -> Arc<ShaderModule> {
        self.vertex_shader.as_ref().unwrap().clone()
    }

    pub fn acquire_pixel_shader_module(&self) -> Option<Arc<ShaderModule>> {
        self.pixel_shader.clone()
    }

    pub fn get_pixel_shader(&self) -> &ShaderModule {
        self.pixel_shader.as_ref().unwrap()
    }

    pub fn clone_pixel_shader(&self) -> Arc<ShaderModule> {
        self.pixel_shader.as_ref().unwrap().clone()
    }
}

impl IShader for ShaderVk {
    type DeviceType = DeviceVk;

    fn new(device: &mut Self::DeviceType, info: &ShaderInfo) -> Self {
        Self::new(device, info)
    }
}
