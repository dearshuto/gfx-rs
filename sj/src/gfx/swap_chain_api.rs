use super::Device;

pub struct SwapChainInfo<'a>
{
    layer: Option<&'a mut super::super::vi::Layer<'a>>,
}

impl <'a> SwapChainInfo<'a>
{
    pub fn new() -> SwapChainInfo<'a>
    {
	SwapChainInfo{
	    layer: None
	}
    }

    pub fn get_layer(&self) -> Option<&'a mut super::super::vi::Layer<'a>>
    {	
	None
    }
    
    pub fn set_layer(mut self, layer: &'a mut super::super::vi::Layer<'a>) -> SwapChainInfo<'a>
    {	
	self.layer = Some(layer);
	self
    }
}

pub trait ISwapChainImpl
{
	fn new(device: &Device, info: &SwapChainInfo) -> Self;
}

pub struct TSwapChain<T>
	where T: ISwapChainImpl
{
	_impl: T,
}

impl<T: ISwapChainImpl> TSwapChain<T>
{
	pub fn new(device: &Device, info: &SwapChainInfo) -> Self
	{
		Self{
			_impl: T::new(device, info),
		}
	}
}
