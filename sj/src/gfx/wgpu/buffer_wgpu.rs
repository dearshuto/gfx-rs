use crate::gfx::buffer_api::{BufferInfo, IBufferImpl};
use crate::gfx::{Device, GpuAccess, MemoryPool};
use std::ops::Deref;
use std::sync::Arc;
use std::marker::PhantomData;
use futures::executor;

pub struct BufferImpl<'a> {
    _device: &'a Device,
    _buffer_impl: Arc<wgpu::Buffer>,
    _size: usize,
    _marker: PhantomData<&'a i32>,
}

impl<'a> IBufferImpl<'a> for BufferImpl<'a> {
    fn new(
        device: &'a Device,
        info: &BufferInfo,
        _memory_pool: &'a MemoryPool,
        _offset: i64,
        _size: u64,
    ) -> Self {
        assert!(info.get_size() <= _size);

        let slice_size = info.get_size();
        let size = slice_size as wgpu::BufferAddress;
        let buffer = device
            .to_data()
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size,
                usage: info.get_as_usage(),
                mapped_at_creation: false,
            });

        BufferImpl {
            _device: device,
            _buffer_impl: Arc::new(buffer),
            _size: info.get_size() as usize,
            _marker: PhantomData,
        }
    }

    fn get_required_alignment(_device: &Device, _info: &BufferInfo) -> u64 {
        0b1
    }

    fn map(&self) {    }

	fn read<TType: 'static>(&self, action: fn(&mut TType)) {
		let slice = self._buffer_impl.slice(..);
		let future = slice.map_async(wgpu::MapMode::Read);
		self._device.to_data().get_device().poll(wgpu::Maintain::Wait);
		let _awaiter = executor::block_on(future).unwrap();
		let mapped_range = slice.get_mapped_range();
		let mapped_data = mapped_range.deref();
		let mut data: TType = unsafe { std::ptr::read(mapped_data.as_ptr() as *const _) };
		action(&mut data);
	}

    fn read_with_user_data<TType: 'static, TUserData>(
        &self,
        action: fn(&mut TType, Option<&mut TUserData>),
        user_data: Option<&mut TUserData>,
    ) {
		let slice = self._buffer_impl.slice(..);
		let future = slice.map_async(wgpu::MapMode::Read);
		self._device.to_data().get_device().poll(wgpu::Maintain::Wait);
		let _awaiter = executor::block_on(future).unwrap();
		let mapped_range = slice.get_mapped_range();
		let mapped_data = mapped_range.deref();
		let mut data: TType = unsafe { std::ptr::read(mapped_data.as_ptr() as *const _) };
		action(&mut data, user_data);
	}

    fn write<TType: 'static>(&self, action: fn(&mut TType)) {
		let slice = self._buffer_impl.slice(..);
		let future = slice.map_async(wgpu::MapMode::Write);
		self._device.to_data().get_device().poll(wgpu::Maintain::Wait);
		let _awaiter = executor::block_on(future).unwrap();
		let mapped_range = slice.get_mapped_range();
		let mapped_data = mapped_range.deref();
		let mut data: TType = unsafe { std::ptr::read(mapped_data.as_ptr() as *const _) };
		action(&mut data);
	}

    fn write_with_user_data<TType: 'static, TUserData>(
        &self,
        _action: fn(&mut TType, Option<&mut TUserData>),
        _user_data: Option<&mut TUserData>,
    ) {
		todo!()
	}


    fn unmap(&self) {}

    fn flush_mapped_range(&self, _offset: i64, _size: u64) {}

    fn invalidate_mapped_range(&self, _offset: i64, _size: u64) {}
}

impl<'a> BufferImpl<'a> {
    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self._buffer_impl
    }

    // pub fn get_buffer_mut(&mut self) -> &mut wgpu::Buffer {
    //     &mut self._buffer_impl
    // }

	pub fn clone_buffer(&self) -> Arc<wgpu::Buffer> {
		self._buffer_impl.clone()
	}
}

impl BufferInfo {
    pub fn get_as_usage(&self) -> wgpu::BufferUsages {
        let gpu_access = self.get_gpu_access_flags();

        let mut result = wgpu::BufferUsages::empty();
        if gpu_access.contains(GpuAccess::VERTEX_BUFFER) {
            result |= wgpu::BufferUsages::VERTEX;
        }
        if gpu_access.contains(GpuAccess::INDEX_BUFFER) {
            result |= wgpu::BufferUsages::INDEX;
        }
        if gpu_access.contains(GpuAccess::UNORDERED_ACCESS_BUFFER) {
            result |= wgpu::BufferUsages::STORAGE;
        }
        if gpu_access.contains(GpuAccess::CONSTANT_BUFFER) {
            result |= wgpu::BufferUsages::UNIFORM;
        }
        if gpu_access.contains(GpuAccess::READ) {
			result |= wgpu::BufferUsages::COPY_SRC;
        }
        if gpu_access.contains(GpuAccess::WRITE) {
			result |= wgpu::BufferUsages::COPY_DST;
        }

		result |= wgpu::BufferUsages::MAP_READ;
		result |= wgpu::BufferUsages::MAP_WRITE;
		
        result
    }
}
