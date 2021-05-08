use super::super::Device;
use super::super::swap_chain_api::{SwapChainInfo, ISwapChainImpl};

pub struct SwapChainImpl
{
	
}

impl ISwapChainImpl for SwapChainImpl
{
	fn new(_device: &Device, _info: &SwapChainInfo) -> Self
	{
		Self{
			
		}
	}
}
