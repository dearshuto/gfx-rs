use super::super::device_info::DeviceInfo;

pub struct Device
{
    
}

impl Device
{
    pub fn new() -> Device
    {
	Device{}
    }

    pub fn initialize(&mut self, info: DeviceInfo)
    {
    }
}

impl Drop for Device
{
    fn drop(&mut self)
    {
    }
}
