mod buffer_wsys;
mod color_target_view_wsys;
mod command_buffer_wsys;
mod depth_stencil_view_wsys;
mod device_ws;
mod fence_wsys;
mod queue_wsys;
mod sampler_wsys;
mod semaphore_wsys;
mod shader_wsys;
mod swap_chain_wsys;
mod texture_view_wsys;
mod texture_wsys;
mod vertex_state_wsys;

pub use buffer_wsys::BufferWsys;
pub use color_target_view_wsys::ColorTargetViewWsys;
pub use command_buffer_wsys::CommandBufferWsys;
pub use depth_stencil_view_wsys::DepthStencilViewWsys;
pub use device_ws::DeviceWsys;
pub use fence_wsys::FenceWsys;
pub use queue_wsys::QueueWsys;
pub use sampler_wsys::SamplerWsys;
pub use semaphore_wsys::SemaphoreWsys;
pub use shader_wsys::ShaderWsys;
pub use swap_chain_wsys::SwapChainWsys;
pub use texture_view_wsys::TextureViewWsys;
pub use texture_wsys::TextureWsys;
pub use vertex_state_wsys::VertexStateWsys;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
