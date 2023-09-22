use sjgfx_interface::ShaderInfo;

use crate::DeviceAsh;

pub struct CommandData {
    pub command_buffer: ash::vk::CommandBuffer,
}

pub struct ShaderAsh {
    handle: ash::extensions::ext::ShaderObject,
    vertex_shader: Option<ash::vk::ShaderEXT>,

    compute_shader: Option<ash::vk::ShaderEXT>,
}

impl ShaderAsh {
    pub fn new(device: &DeviceAsh, info: &ShaderInfo) -> Self {
        // 共通インスタンスが初期化されている
        let Some(shared_data) = crate::SHARED_INSTANCE.get() else {
            panic!()
        };

        let instance = &shared_data.instance;
        let shader_object = ash::extensions::ext::ShaderObject::new(instance, &device.handle());

        // let vertex_shader = Self::create_shader_ext(
        //     shader_object.clone(),
        //     ash::vk::ShaderStageFlags::VERTEX,
        //     Some(info.get_vertex_shader_binary().unwrap()),
        // );
        let vertex_shader = None;
        let compute_shader = Self::create_shader_ext(
            shader_object.clone(),
            ash::vk::ShaderStageFlags::COMPUTE,
            Some(info.get_compute_shader_binary().unwrap()),
        );
        Self {
            handle: shader_object,
            vertex_shader,
            compute_shader,
        }
    }

    pub fn get_shader_object(
        &self,
        shader_stage: sjgfx_interface::ShaderStage,
    ) -> Option<ash::vk::ShaderEXT> {
        match shader_stage {
            sjgfx_interface::ShaderStage::Vertex => self.vertex_shader,
            sjgfx_interface::ShaderStage::Pixel => todo!(),
            sjgfx_interface::ShaderStage::Compute => self.compute_shader,
        }
    }

    pub fn push_command(&self, data: CommandData) {
        let Some(compute_shader) = self.compute_shader else {
            return;
        };

        let command_buffer = data.command_buffer;

        // 演算シェーダー
        unsafe {
            self.handle.cmd_bind_shaders(
                command_buffer,
                &[ash::vk::ShaderStageFlags::COMPUTE],
                &[compute_shader],
            );
        }
    }

    fn create_shader_ext(
        shader_object: ash::extensions::ext::ShaderObject,
        shader_stage_flags: ash::vk::ShaderStageFlags,
        shader_binary: Option<&[u8]>,
    ) -> Option<ash::vk::ShaderEXT> {
        let Some(shader_binary) = shader_binary else {
            return None;
        };

        let shader_ext_create_info = ash::vk::ShaderCreateInfoEXT::builder()
            .stage(shader_stage_flags)
            .code_type(ash::vk::ShaderCodeTypeEXT::SPIRV)
            .code(shader_binary)
            .build();
        let shader_ext = unsafe {
            shader_object
                .create_shaders(&[shader_ext_create_info], None)
                .unwrap()[0]
        };
        Some(shader_ext)
    }
}

impl Drop for ShaderAsh {
    fn drop(&mut self) {
        if let Some(vertex_shader) = self.vertex_shader {
            unsafe { self.handle.destroy_shader(vertex_shader, None) };
        }
    }
}
