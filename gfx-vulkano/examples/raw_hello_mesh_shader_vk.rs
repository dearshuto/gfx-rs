use ash::RawPtr;
use vulkano::{
    command_buffer::{
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
        AutoCommandBufferBuilder, CommandBufferUsage,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo,
        QueueFlags,
    },
    instance::{Instance, InstanceCreateInfo},
    VulkanLibrary, VulkanObject,
};

fn main() {
    let vulkan_library = VulkanLibrary::new().unwrap();
    let instance = Instance::new(
        vulkan_library,
        InstanceCreateInfo {
            ..Default::default()
        },
    )
    .unwrap();

    // 物理デバイスの取得
    let device_ext = vulkano::device::DeviceExtensions::default();
    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| p.supported_extensions().contains(&device_ext))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(_i, q)| {
                    q.queue_flags.contains(QueueFlags::GRAPHICS)
                    //&& p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|q| (p, q as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .unwrap();

    let device_extensions = DeviceExtensions::default();
    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .unwrap();

    let queue = queues.next().unwrap();

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo {
            primary_buffer_count: 1,
            secondary_buffer_count: 1,
            ..Default::default()
        },
    );
    let _command_builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    // これでメッシュシェーダーのバイナリを渡せば ShaderModule ができあがる
    // let shader_module = unsafe { vulkano::shader::ShaderModule::from_bytes(device.clone(), &[]) };

    let _fp = ash::vk::ExtMeshShaderFn::load(|name| unsafe {
        let getter = device
            .physical_device()
            .instance()
            .fns()
            .v1_0
            .get_device_proc_addr;
        let addr = getter(device.handle(), name.as_ptr());
        std::mem::transmute(addr)
    });

    let func = ash::vk::ExtShaderObjectFn::load(|name| unsafe {
        let getter = device
            .physical_device()
            .instance()
            .fns()
            .v1_0
            .get_device_proc_addr;
        let addr = getter(device.handle(), name.as_ptr());
        std::mem::transmute(addr)
    });
    let creator = func.create_shaders_ext;
    let mut shaders = Vec::with_capacity(0);
    let create_infos = [];
    if !create_infos.is_empty() {
        unsafe {
            creator(
                device.handle(),
                create_infos.len() as u32,
                create_infos.as_ptr(),
                None.as_raw_ptr(),
                shaders.as_mut_ptr(),
            )
            .result()
            .unwrap()
        };
    }

    for shader in shaders {
        unsafe { (func.destroy_shader_ext)(device.handle(), shader, None.as_raw_ptr()) };
    }

    println!("Hello World!");
}
