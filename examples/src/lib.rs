use sjgfx_interface::{IBuffer, IDevice};
use sjgfx_util::ObjData;

pub fn load_obj<TDevice, TBuffer>(device: &mut TDevice, obj_text: &str) -> ObjData<TBuffer>
where
    TDevice: IDevice,
    TBuffer: IBuffer<DeviceType = TDevice>,
{
    sjgfx_util::load_obj(device, obj_text)
}
