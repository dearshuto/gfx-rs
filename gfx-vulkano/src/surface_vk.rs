use std::sync::Arc;

use vulkano::swapchain::Surface;
use winit::window::Window;

pub struct SurfaceVk {
    surface: Arc<Surface<Window>>,
}

impl SurfaceVk {
    pub fn clone_surface(&self) -> Arc<Surface<Window>> {
        self.surface.clone()
    }
}
