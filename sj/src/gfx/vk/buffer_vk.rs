use super::super::buffer_info::BufferInfo;

pub struct Buffer
{

}

impl Buffer
{
    pub fn new(device: &super::device_vk::Device, info: &BufferInfo) -> Buffer
    {
	vulkano::buffer::CpuAccessibleBuffer::uninitialized_array(device.clone(), 124, vulkano::buffer::BufferUsage::all(), false);
	Buffer{}	
    }
}
