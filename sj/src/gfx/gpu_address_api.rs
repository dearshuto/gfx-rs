use super::Buffer;

pub trait IGpuAddressImpl<'a> {
    fn new<'buffer: 'a>(buffer: &'buffer Buffer<'buffer>) -> Self;

    fn offset(&mut self, offset: i64);
}

pub struct TGpuAddressInterface<'a, T: 'a>
where
    T: IGpuAddressImpl<'a>,
{
    _impl: T,
    _phantom: std::marker::PhantomData<&'a i8>,
}

impl<'a, T: IGpuAddressImpl<'a>> TGpuAddressInterface<'a, T> {
    pub fn new<'buffer: 'a>(buffer: &'buffer Buffer<'buffer>) -> Self {
        Self {
            _impl: T::new(buffer),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn offset(&mut self, offset: i64) {
        self._impl.offset(offset);
    }

    pub fn to_data(&self) -> &T {
        &self._impl
    }

    pub fn to_data_mut(&mut self) -> &mut T {
        &mut self._impl
    }
}
