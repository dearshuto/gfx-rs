use sjgfx_interface::{BufferInfo, QueueInfo};

pub fn main() {
    sjgfx_ash::initialize();

    {
        let device = sjgfx_ash::DeviceAsh::new(&sjgfx_interface::DeviceInfo::new());
        let memory_pool = sjgfx_ash::MemoryPoolAsh::new(&device);
        let buffer = sjgfx_ash::BufferAsh::new(
            &device,
            BufferInfo::new().set_size(512),
            &memory_pool,
            0,
            512,
        );
        buffer.map_mut(|_data: &mut i32| {});

        let shader = sjgfx_ash::ShaderAsh::new(
            &device,
            &sjgfx_interface::ShaderInfo::new()
                .set_compute_shader_binary(include_bytes!("../outputs/shader.spv")),
        );

        let mut command_buffer =
            sjgfx_ash::CommandBufferAsh::new(&device, &sjgfx_interface::CommandBufferInfo::new());
        command_buffer.set_buffer(&buffer);
        command_buffer.set_shader(&shader);

        let _queue = sjgfx_ash::QueueAsh::new(&device, &QueueInfo::new());
    }
    sjgfx_ash::finalize();
}