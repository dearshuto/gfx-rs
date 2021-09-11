use vulkano::buffer::BufferAccess;

use crate::gfx::{IndexFormat, PrimitiveTopology};

pub enum DrawCommand
{
	Simple(SimpleDrawCommandInfo),
	Instanced(InstancedDrawCommandInfo),
	Indexed(IndexedDrawCommandInfo),
	IndexedInstancing(IndexedInstancedDrawInfo),
}

pub struct SimpleDrawCommandInfo
{
	_primitive_topology: PrimitiveTopology,
	_vertex_count: i32,
	_vertex_offet: i32,
}

impl SimpleDrawCommandInfo
{
	pub fn new(primitive_topology: PrimitiveTopology, vertex_count: i32, vertex_offet: i32) -> Self {
		Self {
			_primitive_topology: primitive_topology,
			_vertex_count: vertex_count,
			_vertex_offet: vertex_offet,
		}
	}			
}

pub struct InstancedDrawCommandInfo
{
	_primitive_topology: PrimitiveTopology,
	_vertex_count: i32,
	_vertex_offset: i32,
	_instance_count: i32,
	_base_instance: i32,
}

impl InstancedDrawCommandInfo
{
	pub fn new(primitive_topology: crate::gfx::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,) -> Self {
		Self {
			_primitive_topology: primitive_topology,
			_vertex_count: vertex_count,
			_vertex_offset: vertex_offset,
			_instance_count: instance_count,
			_base_instance: base_instance
		}
	}
}

pub struct IndexedDrawCommandInfo
{
	_primitive_topology: PrimitiveTopology,
	_index_format: IndexFormat,
	_index_buffer: std::sync::Arc<dyn BufferAccess>,
	_index_count: i32,
	_base_vertex: i32,
}

impl IndexedDrawCommandInfo {
	pub fn new(
		primitive_topology: PrimitiveTopology,
	index_format: IndexFormat,
	index_buffer: std::sync::Arc<dyn BufferAccess>,
	index_count: i32,
	base_vertex: i32) -> Self {
		Self {
			_primitive_topology: primitive_topology,
			_index_format: index_format,
			_index_buffer: index_buffer,
			_index_count: index_count,
			_base_vertex: base_vertex
		}
	}
}

pub struct IndexedInstancedDrawInfo
{
	_primitive_topology: PrimitiveTopology,
	_index_format: IndexFormat,
	_index_buffer: std::sync::Arc<dyn BufferAccess>,
	_index_count: i32,
	_base_vertex: i32,
	_instance_count: i32,
    _base_instance: i32,
}

impl IndexedInstancedDrawInfo {
	pub fn new(
		primitive_topology: PrimitiveTopology,
		index_format: IndexFormat,
		index_buffer: std::sync::Arc<dyn BufferAccess>,
		index_count: i32,
		base_vertex: i32,
		instance_count: i32,
		base_instance: i32,

	) -> Self {
		Self {
			_primitive_topology: primitive_topology,
			_index_format: index_format,
			_index_buffer: index_buffer,
			_index_count: index_count,
			_base_vertex: base_vertex,
			_instance_count: instance_count,
			_base_instance: base_instance
		}
	}
}
