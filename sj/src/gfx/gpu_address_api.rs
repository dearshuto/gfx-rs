pub trait IGpuAddressImpl<'a> {
    fn offset(&mut self, offset: i64);
}

pub struct TGpuAddressInterface<'a, T>
where
    T: IGpuAddressImpl<'a>,
{
    _impl: T,
    _phantom: std::marker::PhantomData<&'a i8>,
}

impl<'a, T: IGpuAddressImpl<'a>> TGpuAddressInterface<'a, T> {
    pub fn new(impl_instance: T) -> Self {
        Self {
            _impl: impl_instance,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn offset(&mut self, offset: i64) {
        self._impl.offset(offset);
    }

    pub fn to_data(&self) -> &T {
        &self._impl
    }

    pub fn to_data_mut(&'a mut self) -> &'a mut T {
        &mut self._impl
    }
}
