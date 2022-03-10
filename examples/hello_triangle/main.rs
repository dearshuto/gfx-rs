use sjgfx_interface::{IDevice, DeviceInfo, IQueue, ICommandBuffer, QueueInfo, CommandBufferInfo, IBuffer, BufferInfo, GpuAccess, PrimitiveTopology};
use sjgfx_wgpu::{DeviceWgpu, QueueWgpu, CommandBufferWgpu, BufferWgpu};
use sjgfxex::IAppLogic;

fn main() {
    let _device = DeviceWgpu::new(&DeviceInfo::new());
    //let mut hello_triangle = HelloTriangle::<QueueWgpu, CommandBufferWgpu, BufferWgpu>::new(&device);
    sjgfxex::App::run::<HelloTriangle<QueueWgpu, CommandBufferWgpu, BufferWgpu>>();
}

#[repr(C)]
struct HelloTriangle<'a, TQueue, TCommandBuffer, TBuffer>
    where TQueue: IQueue<'a>, TCommandBuffer: ICommandBuffer<'a>, TBuffer: IBuffer<'a>
{
    queue: TQueue,
    command_buffer: TCommandBuffer,
    vertex_buffer: TBuffer,
    _marker: std::marker::PhantomData<&'a()>
}

impl<'a, TDevice, TQueue, TCommandBuffer, TBuffer> HelloTriangle<'a, TQueue, TCommandBuffer, TBuffer>
where TDevice: IDevice, TQueue: IQueue<'a, DeviceType = TDevice>, TCommandBuffer: ICommandBuffer<'a, DeviceType = TDevice>, TBuffer: IBuffer<'a, DeviceType = TDevice>
{
    pub fn new(device: &'a TDevice) -> Self {
        let d = TDevice::new(&DeviceInfo::new());
        let queue = TQueue::new(&device, &QueueInfo::new());
        let command_buffer = TCommandBuffer::new(device, &CommandBufferInfo::new());
        let vertex_buffer = TBuffer::new(device, &BufferInfo::new().set_gpu_access_flags(GpuAccess::VERTEX_BUFFER).set_size(128));

        let queue = TQueue::new(&d, &QueueInfo::new());
        let command_buffer = TCommandBuffer::new(&d, &CommandBufferInfo::new());
        let vertex_buffer = TBuffer::new(&d, &BufferInfo::new().set_gpu_access_flags(GpuAccess::VERTEX_BUFFER).set_size(128));

        Self{
            queue,
            command_buffer,
            vertex_buffer,
            _marker: std::marker::PhantomData
        }
    }
}

impl<'a, TQueue, TCommandBuffer, TBuffer> IAppLogic<'a> for HelloTriangle<'a, TQueue, TCommandBuffer, TBuffer>
where TQueue: IQueue<'a, CommandBufferType = TCommandBuffer>, TCommandBuffer: ICommandBuffer<'a, BufferType = TBuffer>, TBuffer: IBuffer<'a>
{
    fn new() -> Self {
        todo!()
    }

    fn update(&mut self) { }

    fn draw(&'a mut self) {
        println!("Draw");

        self.command_buffer.begin();
        self.command_buffer.set_vertex_buffer(0, &self.vertex_buffer);
        self.command_buffer.draw(PrimitiveTopology::TriangleList, 6/*count*/, 0/*vertex_offset*/);
        self.command_buffer.enf();

        self.queue.execute(&self.command_buffer);
        self.queue.flush();
        self.queue.sync();
    }
}
