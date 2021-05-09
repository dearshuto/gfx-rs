use winit::platform::run_return::EventLoopExtRunReturn;
use super::super::Device;
use super::super::super::vi::Layer;
use super::super::swap_chain_api::{SwapChainInfo, ISwapChainImpl};

pub struct SwapChainImpl<'a>
{
	_layer: Layer<'a>,
}

impl<'a> SwapChainImpl<'a>
{
}

impl<'a, 'ref_layer: 'a, 'layer: 'ref_layer> ISwapChainImpl<'a, 'ref_layer, 'layer> for SwapChainImpl<'a>
{
	fn new(_device: &Device, info: SwapChainInfo<'a>) -> Self
	{
		Self{
			_layer: info.get_layer(),
		}
	}

	fn update(&mut self) {
		let event_loop = &mut self._layer.get_event_loop_mut();

		event_loop.run_return(|event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Wait;

            if let winit::event::Event::WindowEvent { event: _, .. } = &event {
                // Print only Window events to reduce noise
                //println!("{:?}", event);
            }

            match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    //quit = true;
                }
                winit::event::Event::MainEventsCleared => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                _ => (),
            }
        });
	}
}
