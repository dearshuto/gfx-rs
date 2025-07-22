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
        unsafe { self.enumerate_physical_devices() }
    }

    fn get_physical_device_queue_family_properties(
        &self,
        physical_device: Self::PhysicalDevice,
    ) -> Vec<ash::vk::QueueFamilyProperties> {
        unsafe { self.get_physical_device_queue_family_properties(physical_device) }
    }

    fn create_device(
        &self,
        physical_device: ash::vk::PhysicalDevice,
        create_info: &ash::vk::DeviceCreateInfo,
        allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::Device, ash::vk::Result> {
        unsafe { self.create_device(physical_device, create_info, allocator) }
    }
}

impl Device for ash::Device {
    type Queue = ash::vk::Queue;
    type CommandPool = ash::vk::CommandPool;
    type CommandBuffer = ash::vk::CommandBuffer;
    type ImageView = ash::vk::ImageView;
    type RenderPass = ash::vk::RenderPass;
    type Pipeline = ash::vk::Pipeline;
    type Semaphore = ash::vk::Semaphore;
    type Fence = ash::vk::Fence;

    fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> Self::Queue {
        unsafe { self.get_device_queue(queue_family_index, queue_index) }
    }

    fn create_command_pool(
        &self,
        create_info: &ash::vk::CommandPoolCreateInfo,
        allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::CommandPool, ash::vk::Result> {
        unsafe { self.create_command_pool(create_info, allocator) }
    }

    fn allocate_command_buffers(
        &self,
        allocate_info: &ash::vk::CommandBufferAllocateInfo,
    ) -> Result<Vec<Self::CommandBuffer>, ash::vk::Result> {
        unsafe { self.allocate_command_buffers(allocate_info) }
    }

    fn create_image_view(
        &self,
        create_info: &ash::vk::ImageViewCreateInfo,
        allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::ImageView, ash::vk::Result> {
        unsafe { self.create_image_view(create_info, allocator) }
    }

    fn cmd_begin_render_pass(
        &self,
        command_buffer: Self::CommandBuffer,
        begin_info: &ash::vk::RenderPassBeginInfo,
        contents: ash::vk::SubpassContents,
    ) {
        unsafe { self.cmd_begin_render_pass(command_buffer, begin_info, contents) };
    }

    fn cmd_bind_pipeline(
        &self,
        command_buffer: Self::CommandBuffer,
        bind_point: ash::vk::PipelineBindPoint,
        pipeline: Self::Pipeline,
    ) {
        unsafe { self.cmd_bind_pipeline(command_buffer, bind_point, pipeline) };
    }

    fn cmd_draw(
        &self,
        command_buffer: Self::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            self.cmd_draw(
                command_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        };
    }

    fn cmd_end_render_pass(&self, command_buffer: Self::CommandBuffer) {
        unsafe { self.cmd_end_render_pass(command_buffer) };
    }
}
