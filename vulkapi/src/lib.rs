mod ash_bindings;
mod wgpu_bindings;

pub trait Interface {
    type Entry: Entry;
}

pub trait Entry: Sized {
    type Instance;

    fn loaded() -> Result<Self, ash::LoadingError>;

    fn create_instance(
        &self,
        create_info: &ash::vk::InstanceCreateInfo,
        allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::Instance, ash::vk::Result>;
}

pub trait Instance {
    type PhysicalDevice: Copy;
    type Device: Device;

    fn enumerate_physical_devices(&self) -> Result<Vec<Self::PhysicalDevice>, ash::vk::Result>;

    fn get_physical_device_queue_family_properties(
        &self,
        physical_device: Self::PhysicalDevice,
    ) -> Vec<ash::vk::QueueFamilyProperties>;

    fn create_device(
        &self,
        physical_device: ash::vk::PhysicalDevice,
        create_info: &ash::vk::DeviceCreateInfo,
        allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::Device, ash::vk::Result>;
}

pub trait Device {
    type Queue: Copy;
    type CommandPool;
    type CommandBuffer;
    type ImageView;
    type RenderPass: Copy;
    type Pipeline: Copy;
    type Semaphore: Copy;
    type Fence: Copy;

    fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> Self::Queue;

    fn create_command_pool(
        &self,
        create_info: &ash::vk::CommandPoolCreateInfo,
        allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::CommandPool, ash::vk::Result>;

    fn allocate_command_buffers(
        &self,
        allocate_info: &ash::vk::CommandBufferAllocateInfo,
    ) -> Result<Vec<Self::CommandBuffer>, ash::vk::Result>;

    fn create_image_view(
        &self,
        create_info: &ash::vk::ImageViewCreateInfo,
        allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::ImageView, ash::vk::Result>;

    // 描画コマンド

    fn cmd_begin_render_pass(
        &self,
        command_buffer: Self::CommandBuffer,
        begin_info: &ash::vk::RenderPassBeginInfo,
        contents: ash::vk::SubpassContents,
    );

    fn cmd_bind_pipeline(
        &self,
        command_buffer: Self::CommandBuffer,
        bind_point: ash::vk::PipelineBindPoint,
        pipeline: Self::Pipeline,
    );

    fn cmd_draw(
        &self,
        command_buffer: Self::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );

    fn cmd_end_render_pass(&self, command_buffer: Self::CommandBuffer);
}
