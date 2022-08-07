use sjgfx_interface::{
    IBuffer, IColorTargetView, ICommandBuffer, IDepthStencilView, IDevice, IFence, IQueue,
    ISampler, ISemaphore, IShader, ISwapChain, ITexture, ITextureView, IVertexState,
};
use sjvi::{IDisplay, IInstance};

#[cfg(not(target_arch = "wasm32"))]
mod ash;

#[cfg(not(target_arch = "wasm32"))]
mod glow;

#[cfg(not(target_arch = "wasm32"))]
mod vulkano;

#[cfg(not(target_arch = "wasm32"))]
mod wgpu;

// wasm 用のクレートだけど wasm 以外でもビルドは通るので cfg なし
mod wsys;

#[cfg(not(target_arch = "wasm32"))]
pub use ash::Ash;

#[cfg(not(target_arch = "wasm32"))]
pub use glow::Glow;

#[cfg(not(target_arch = "wasm32"))]
pub use vulkano::Vulkano;

#[cfg(not(target_arch = "wasm32"))]
pub use wgpu::Wgpu;

pub use wsys::Wsys;

pub trait IApi {
    type Buffer: IBuffer<DeviceType = Self::Device>;
    type ColorTargetView: IColorTargetView<DeviceType = Self::Device, TextureType = Self::Texture>;
    type DepthStencilView: IDepthStencilView<DeviceType = Self::Device, TextureType = Self::Texture>;
    type Device: IDevice<Display = Self::Display>;
    type Queue: IQueue<
        DeviceType = Self::Device,
        CommandBufferType = Self::CommandBuffer,
        SwapChainType = Self::SwapChain,
    >;
    type CommandBuffer: ICommandBuffer<
        DeviceType = Self::Device,
        DepthStencilViewType = Self::DepthStencilView,
        ShaderType = Self::Shader,
        BufferType = Self::Buffer,
        ColorTargetViewType = Self::ColorTargetView,
        SamplerType = Self::Sampler,
        TextureType = Self::Texture,
        TextureViewType = Self::TextureView,
        VertexStateType = Self::VertexState,
    >;
    type Fence: IFence<DeviceType = Self::Device>;
    type Shader: IShader<DeviceType = Self::Device>;
    type Texture: ITexture<DeviceType = Self::Device>;
    type TextureView: ITextureView<DeviceType = Self::Device, TextureType = Self::Texture>;
    type Sampler: ISampler<DeviceType = Self::Device>;
    type Semaphore: ISemaphore<DeviceType = Self::Device>;
    type SwapChain: ISwapChain<
        DeviceType = Self::Device,
        SemaphoreType = Self::Semaphore,
        ColorTargetViewType = Self::ColorTargetView,
    >;
    type VertexState: IVertexState<DeviceType = Self::Device>;

    type Instance: IInstance<Display = Self::Display>;
    type Display: IDisplay;
}
