use crate::{Device, Entry, Instance};

impl Entry for ash::Entry {
    type Instance = ash::Instance;

    fn loaded() -> Result<Self, ash::LoadingError> {
        unsafe { ash::Entry::load() }
    }

    fn create_instance(
        &self,
        create_info: &ash::vk::InstanceCreateInfo,
        allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::Instance, ash::vk::Result> {
        unsafe { self.create_instance(create_info, allocator) }
    }
}

impl Instance for ash::Instance {
    type PhysicalDevice = ash::vk::PhysicalDevice;
    type Device = ash::Device;

    fn enumerate_physical_devices(&self) -> Result<Vec<Self::PhysicalDevice>, ash::vk::Result> {
        todo!()
    }

    fn get_physical_device_queue_family_properties(
        &self,
        physical_device: Self::PhysicalDevice,
    ) -> Vec<ash::vk::QueueFamilyProperties> {
        todo!()
    }

    fn create_device(
        physical_device: ash::vk::PhysicalDevice,
        create_info: &ash::vk::DeviceCreateInfo,
        allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::Device, ash::vk::Result> {
        todo!()
    }
}

impl Device for ash::Device {
    type Queue = ash::vk::Queue;
    type CommandPool = ash::vk::CommandPool;
    type CommandBuffer = ash::vk::CommandBuffer;
    type ImageView;
    type RenderPass;
    type Pipeline;
    type Semaphore;
    type Fence;

    fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> Self::Queue {
        todo!()
    }

    fn create_command_pool(
        &self,
        create_info: &ash::vk::CommandPoolCreateInfo,
        allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Self::CommandPool {
        todo!()
    }

    fn allocate_command_buffers(
        &self,
        allocate_info: &ash::vk::CommandBufferAllocateInfo,
    ) -> Result<Vec<Self::CommandPool>, ash::vk::Result> {
        todo!()
    }

    fn create_image_view(
        &self,
        create_info: &ash::vk::ImageCreateInfo,
        allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Vec<Self::ImageView>, ash::vk::Result> {
        todo!()
    }

    fn cmd_begin_render_pass(
        &self,
        command_buffer: Self::CommandBuffer,
        begin_info: &ash::vk::RenderPassBeginInfo,
        contents: ash::vk::SubpassContents,
    ) {
        todo!()
    }

    fn cmd_bind_pipeline(
        &self,
        command_buffer: Self::CommandBuffer,
        bind_point: ash::vk::PipelineBindPoint,
        pipeline: Self::Pipeline,
    ) {
        todo!()
    }

    fn cmd_draw(&self) {
        todo!()
    }

    fn cmd_end_render_pass(&self, command_buffer: Self::CommandBuffer) {
        todo!()
    }
}
