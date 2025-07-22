use crate::{Device, Entry, Instance};

pub struct EntryAdapter {}

impl Entry for EntryAdapter {
    type Instance = wgpu::Instance;

    fn loaded() -> Result<Self, ash::LoadingError> {
        Ok(Self {})
    }

    fn create_instance(
        &self,
        #[allow(unused)] create_info: &ash::vk::InstanceCreateInfo,
        #[allow(unused)] allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::Instance, ash::vk::Result> {
        Ok(wgpu::Instance::default())
    }
}

struct InstanceAdapter<'a> {
    instance: wgpu::Instance,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Instance for InstanceAdapter<'a> {
    type PhysicalDevice = PhysicalDeviceAdapter;
    type Device = DeviceAdapter<'a>;

    fn enumerate_physical_devices(&self) -> Result<Vec<Self::PhysicalDevice>, ash::vk::Result> {
        Ok(Vec::default())
    }

    fn get_physical_device_queue_family_properties(
        &self,
        #[allow(unused)] physical_device: Self::PhysicalDevice,
    ) -> Vec<ash::vk::QueueFamilyProperties> {
        Vec::default()
    }

    fn create_device(
        &self,
        #[allow(unused)] physical_device: ash::vk::PhysicalDevice,
        #[allow(unused)] create_info: &ash::vk::DeviceCreateInfo,
        #[allow(unused)] allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::Device, ash::vk::Result> {
        todo!()
        // self.request_adapter(options);
    }
}

impl<'a> Device for DeviceAdapter<'a> {
    type Queue = &'a wgpu::Queue;
    type CommandPool = ();
    type CommandBuffer = &'a mut CommandBufferAdapter<'a>;
    type ImageView = wgpu::TextureView;
    type RenderPass = RenderPassAdapter;
    type Pipeline = PipelineAdapter;
    type Semaphore = ();
    type Fence = ();

    fn get_device_queue(
        &self,
        #[allow(unused)] queue_family_index: u32,
        #[allow(unused)] queue_index: u32,
    ) -> Self::Queue {
        todo!()
    }

    fn create_command_pool(
        &self,
        #[allow(unused)] create_info: &ash::vk::CommandPoolCreateInfo,
        #[allow(unused)] allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::CommandPool, ash::vk::Result> {
        todo!()
    }

    fn allocate_command_buffers(
        &self,
        #[allow(unused)] allocate_info: &ash::vk::CommandBufferAllocateInfo,
    ) -> Result<Vec<Self::CommandBuffer>, ash::vk::Result> {
        Ok(Vec::default())
    }

    fn create_image_view(
        &self,
        #[allow(unused)] create_info: &ash::vk::ImageViewCreateInfo,
        #[allow(unused)] allocator: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<Self::ImageView, ash::vk::Result> {
        todo!()
    }

    fn cmd_begin_render_pass(
        &self,
        #[allow(unused)] command_buffer: Self::CommandBuffer,
        #[allow(unused)] begin_info: &ash::vk::RenderPassBeginInfo,
        #[allow(unused)] contents: ash::vk::SubpassContents,
    ) {
        // let mut command_encoder = self
        //     .device
        //     .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        // let render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {});

        // command_buffer.render_pass = Some(render_pass);
    }

    fn cmd_bind_pipeline(
        &self,
        #[allow(unused)] command_buffer: Self::CommandBuffer,
        #[allow(unused)] bind_point: ash::vk::PipelineBindPoint,
        #[allow(unused)] pipeline: Self::Pipeline,
    ) {
        todo!()
    }

    fn cmd_draw(
        &self,
        command_buffer: Self::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        let Some(command_encoder) = &mut command_buffer.render_pass else {
            return;
        };

        command_encoder.draw(
            first_vertex..(first_vertex + vertex_count),
            first_instance..(first_instance + instance_count),
        );
    }

    fn cmd_end_render_pass(&self, command_buffer: Self::CommandBuffer) {
        command_buffer.render_pass = None;
    }
}

struct DeviceAdapter<'a> {
    device: wgpu::Device,
    _marker: std::marker::PhantomData<&'a ()>,
}

#[derive(Debug, Clone, Copy)]
struct PhysicalDeviceAdapter {}

#[derive(Debug, Clone, Copy)]
struct PipelineAdapter {}

#[derive(Debug, Clone, Copy)]
struct RenderPassAdapter {}

struct CommandBufferAdapter<'a> {
    render_pass: Option<wgpu::RenderPass<'a>>,
}
