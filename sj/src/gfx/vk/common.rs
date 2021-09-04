use std::default::Default;

pub trait VertexInput {}

#[derive(Default, Debug, Clone)]
pub struct Float3232 {
    pub p0: [f32; 2],
}
vulkano::impl_vertex!(Float3232, p0);
impl VertexInput for Float3232 {}

pub struct Float3232Float3232 {
    pub p0: [f32; 2],
    pub p1: [f32; 2],
}
impl VertexInput for Float3232Float3232 {}

pub struct Float323232Float323232 {
    pub p0: [f32; 3],
    pub p1: [f32; 3],
}
impl VertexInput for Float323232Float323232 {}

pub struct Float323232Float323232Float3232 {
    pub p0: [f32; 3],
    pub p1: [f32; 3],
    pub p2: [f32; 2],
}
impl VertexInput for Float323232Float323232Float3232 {}

//
//

pub trait IData {}

pub struct Data64 {
    buffer: [u8; 64],
}
//vulkano::impl_vertex!(Data64, buffer);
impl IData for Data64 {}

pub struct Data128 {
    pub buffer: [u8; 128],
}
impl IData for Data128 {}

pub struct Data256 {
    pub buffer: [u8; 256],
}
impl IData for Data256 {}

pub struct Data512 {
    pub buffer: [u8; 512],
}
impl IData for Data512 {}

pub struct Data1024 {
    pub buffer: [u8; 1024],
}
impl IData for Data1024 {}

pub struct Data2048 {
    pub buffer: [u8; 2048],
}
impl IData for Data2048 {}

pub struct Data4096 {
    pub buffer: [u8; 4096],
}
impl IData for Data4096 {}
