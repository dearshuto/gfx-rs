use sjgfx_interface::CommandBufferInfo;

use crate::{BufferAsh, DeviceAsh, ShaderAsh};

pub struct CommandBufferAsh {
    command_pool: ash::vk::CommandPool,

    handle: ash::vk::CommandBuffer,

    device: ash::Device,
}

impl CommandBufferAsh {
    pub fn new(device: &DeviceAsh, _info: &CommandBufferInfo) -> Self {
        let command_pool_create_info = ash::vk::CommandPoolCreateInfo::builder()
            .queue_family_index(0)
            .flags(ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .build();
        let command_pool = unsafe {
            device
                .handle()
                .create_command_pool(&command_pool_create_info, None)
        }
        .unwrap();

        let command_buffer_create_info = ash::vk::CommandBufferAllocateInfo::builder()
            .command_pool(command_pool)
            .command_buffer_count(1)
            .build();
        let command_buffers = unsafe {
            device
                .handle()
                .allocate_command_buffers(&command_buffer_create_info)
        }
        .unwrap();

        Self {
            command_pool,
            handle: command_buffers[0],
            device: device.handle(),
        }
    }

    pub fn set_shader(&mut self, shader: &ShaderAsh) {
        shader.push_command(crate::shader_ash::CommandData {
            command_buffer: self.handle,
        })
    }

    pub fn set_buffer(&mut self, _buffer: &BufferAsh) {}

    pub fn dispatch(&mut self, count_x: i32, count_y: i32, count_z: i32) {
        unsafe {
            self.device
                .cmd_dispatch(self.handle, count_x as u32, count_y as u32, count_z as u32);
        }
    }

    pub fn handle(&self) -> ash::vk::CommandBuffer {
        self.handle
    }
}

impl Drop for CommandBufferAsh {
    fn drop(&mut self) {
        unsafe {
            self.device
                .free_command_buffers(self.command_pool, &[self.handle]);
            self.device.destroy_command_pool(self.command_pool, None);
        }
    }
}
