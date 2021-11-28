pub trait IScanBufferView {}

pub struct TScanBufferView<T: IScanBufferView> {
    _impl: T,
}

impl<T: IScanBufferView> TScanBufferView<T> {
    pub fn new(instance: T) -> Self {
        Self { _impl: instance }
    }

    pub fn to_data(&self) -> &T {
        &self._impl
    }

    pub fn move_data(self) -> T {
        self._impl
    }
}
