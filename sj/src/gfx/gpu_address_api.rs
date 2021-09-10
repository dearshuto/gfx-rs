use super::Buffer;

pub trait IGpuAddressImpl<'a> {
    fn new<'buffer: 'a, TType: 'static>(buffer: &'buffer Buffer<'buffer, TType>) -> Self;

    fn offset(&mut self, offset: i64);
}

pub struct TGpuAddressInterface<'a, T: 'a>
where
    T: IGpuAddressImpl<'a>,
{
    _impl: T,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, T: IGpuAddressImpl<'a>> TGpuAddressInterface<'a, T> {
    pub fn new<'buffer: 'a, TType: 'static>(buffer: &'buffer Buffer<'buffer, TType>) -> Self {
        Self {
            _impl: T::new(buffer),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn offset(&mut self, offset: i64) {
        self._impl.offset(offset);
    }

    pub fn to_data(&'a self) -> &'a T {
        &self._impl
    }

    pub fn to_data_mut(&'a mut self) -> &'a mut T {
        &mut self._impl
    }
}
