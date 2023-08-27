use crate::{DeviceAsh, ShaderAsh};

pub struct FenceAsh {
    handle: ash::vk::Fence,

    shader: Option<ShaderAsh>,
}

impl FenceAsh {
    pub fn new(device: &DeviceAsh) -> Self {
        let fence_create_info = ash::vk::FenceCreateInfo::builder()
            .flags(ash::vk::FenceCreateFlags::default())
            .build();
        let handle = unsafe { device.handle().create_fence(&fence_create_info, None) }.unwrap();
        Self { handle }
    }

    pub fn begin(&self) {}

    pub fn end(&self) {}

    pub fn handle(&self) -> ash::vk::Fence {
        self.handle.clone()
    }
}
