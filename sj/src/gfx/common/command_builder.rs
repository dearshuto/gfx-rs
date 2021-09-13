use crate::gfx::{Pipeline, ViewportScissorState};

pub enum CommandBuilder<'a, TGraphics, TCompute>
where
    TGraphics: IGraphicsCommandBuilder<'a>,
    TCompute: IComputeCommandBuilder<'a>,
{
    Graphics(TGraphics),
    Compute(TCompute),

	#[allow(dead_code)]
    Phantom(std::marker::PhantomData<&'a ()>),
}

impl<'a, TGraphics, TCompute> CommandBuilder<'a, TGraphics, TCompute>
where
    TGraphics: IGraphicsCommandBuilder<'a>,
    TCompute: IComputeCommandBuilder<'a>,
{
	pub fn build(&mut self) {
		match self {
			Self::Graphics(ref mut builder) => builder.build(),
			Self::Compute(ref mut builder) => builder.build(),
			Self::Phantom(ref _marker) => panic!(),
		}
	}
	
    pub fn set_viewport_scissor_state(&mut self, viewport_scissor_state: &'a ViewportScissorState) {
        match self {
            Self::Graphics(ref mut builder) => {
                builder.set_viewport_scissor_state(viewport_scissor_state)
            }
            Self::Compute(ref _builder) => panic!(),
            Self::Phantom(ref _marker) => panic!(),
        }
    }

    pub fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: usize,
    ) {
        match self {
            Self::Graphics(ref mut builder) => {
                builder.set_constant_buffer(slot, stage, gpu_address, size)
            }
            Self::Compute(ref mut builder) => {
                builder.set_constant_buffer(slot, stage, gpu_address, size)
            }
            Self::Phantom(ref _marker) => panic!(),
        }
    }

    pub fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: u64,
    ) {
        match self {
            Self::Graphics(ref mut builder) => {
                builder.set_unordered_access_buffer(slot, stage, gpu_address, size)
            }
            Self::Compute(ref mut builder) => {
                builder.set_unordered_access_buffer(slot, stage, gpu_address, size)
            }
            Self::Phantom(ref _marker) => panic!(),
        }
    }

    pub fn set_render_targets(
        &mut self,
        color_target_views: &[&crate::gfx::ColorTargetView],
        depth_stencil_state_view: Option<&crate::gfx::DepthStencilView>,
    ) {
        match self {
            Self::Graphics(ref mut builder) => {
                builder.set_render_targets(color_target_views, depth_stencil_state_view)
            }
            Self::Compute(ref _builder) => panic!(),
            Self::Phantom(ref _g) => panic!(),
        };
    }

    pub fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &crate::gfx::GpuAddress) {
        match self {
            Self::Graphics(ref mut builder) => builder.set_vertex_buffer(buffer_index, gpu_address),
            Self::Compute(ref _builder) => panic!(),
            Self::Phantom(ref _marker) => panic!(),
        }
    }

    pub fn draw(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        match self {
            Self::Graphics(ref mut builder) => {
                builder.draw(primitive_topology, vertex_count, vertex_offset)
            }
            Self::Compute(ref _builder) => panic!(),
            Self::Phantom(ref _marker) => panic!(),
        }
    }

    pub fn draw_instanced(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        match self {
            Self::Graphics(ref mut builder) => builder.draw_instanced(
                primitive_topology,
                vertex_count,
                vertex_offset,
                instance_count,
                base_instance,
            ),
            Self::Compute(ref _builder) => panic!(),
            Self::Phantom(ref _marker) => panic!(),
        }
    }

    pub fn draw_indexed(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        index_format: crate::gfx::IndexFormat,
        gpu_address: &crate::gfx::GpuAddress,
        index_count: i32,
        base_vertex: i32,
    ) {
        match self {
            Self::Graphics(ref mut builder) => builder.draw_indexed(
                primitive_topology,
                index_format,
                gpu_address,
                index_count,
                base_vertex,
            ),
            Self::Compute(ref _builder) => panic!(),
            Self::Phantom(ref _marker) => panic!(),
        }
    }

    pub fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        match self {
            Self::Graphics(ref mut _builder) => panic!(),
            Self::Compute(ref mut builder) => {
                builder.dispatch(group_count_x, group_count_y, group_count_z)
            }
            Self::Phantom(ref _builder) => panic!(),
        }
    }
}

pub trait IGraphicsCommandBuilder<'a> {
    fn build(&mut self);

    fn set_viewport_scissor_state(&mut self, viewport_scissor_state: &'a ViewportScissorState);

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: usize,
    );

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: u64,
    );

    fn set_render_targets(
        &mut self,
        color_target_views: &[&crate::gfx::ColorTargetView],
        depth_stencil_state_view: Option<&crate::gfx::DepthStencilView>,
    );

    fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &crate::gfx::GpuAddress);

    fn draw(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    );

    fn draw_instanced(
        &mut self,
        _primitive_topology: crate::gfx::PrimitiveTopology,
        _vertex_count: i32,
        _vertex_offset: i32,
        _instance_count: i32,
        _base_instance: i32,
    );

    fn draw_indexed(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        index_format: crate::gfx::IndexFormat,
        gpu_address: &crate::gfx::GpuAddress,
        index_count: i32,
        base_vertex: i32,
    );

    fn draw_indexed_instanced(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        index_format: crate::gfx::IndexFormat,
        gpu_address: &crate::gfx::GpuAddress,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    );
}

pub trait IComputeCommandBuilder<'a> {
    fn build(&mut self);

    fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>);

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: usize,
    );

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: u64,
    );

    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32);
}
