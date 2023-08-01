mod mandelbrot;
mod triangle;
use std::sync::Arc;

pub use mandelbrot::Mandelbrot;
pub use triangle::Triangle;

pub enum DemoType {
    None,
    Triangle,
}

pub struct DemoManager<'a> {
    demo_type: DemoType,
    triangle: Demo<'a, Triangle<'a>>,
}

impl<'a> DemoManager<'a> {
    pub fn new(device: Arc<wgpu::Device>, target: wgpu::TextureFormat) -> Self {
        Self {
            demo_type: DemoType::Triangle,
            triangle: Demo::<Triangle>::new(device, target),
        }
    }

    pub fn switch(&mut self, demo_type: DemoType) {
        self.demo_type = demo_type;
    }

    pub fn update(&mut self) {
        match self.demo_type {
            DemoType::None => {}
            DemoType::Triangle => self.triangle.update(),
        }
    }

    pub fn draw(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        match self.demo_type {
            DemoType::None => {}
            DemoType::Triangle => self.triangle.draw(render_pass),
        }
    }
}

trait IDemoImpl<'a> {
    fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self;

    fn update(&mut self);

    fn draw(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}

struct Demo<'a, TResource: IDemoImpl<'a>> {
    device: Arc<wgpu::Device>,
    resource: Option<TResource>,
    format: wgpu::TextureFormat,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, TDemoImpl: IDemoImpl<'a>> Demo<'a, TDemoImpl> {
    pub fn new(device: Arc<wgpu::Device>, target_format: wgpu::TextureFormat) -> Self {
        Self {
            device,
            resource: None,
            format: target_format,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn update(&mut self) {
        if self.resource.is_none() {
            let demo = TDemoImpl::new(&self.device, self.format);

            self.resource = Some(demo);
        }

        self.resource.as_mut().unwrap().update();
    }

    pub fn draw(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.resource.as_ref().unwrap().draw(render_pass);
    }
}
