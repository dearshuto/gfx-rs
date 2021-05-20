pub struct ColorTargetViewInfo
{
	
}

pub trait IColorTargetViewImpl
{
	
}

pub struct TColorTargetView<T>
	where T: IColorTargetViewImpl
{
	_impl: T,
}

impl<T: IColorTargetViewImpl> TColorTargetView<T>
{
	
}
