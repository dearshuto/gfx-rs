use std::sync::Arc;

use vulkano::swapchain::Surface;

pub struct SurfaceVk {
    surface: Arc<Surface>,
}

impl SurfaceVk {
    pub fn clone_surface(&self) -> Arc<Surface> {
        self.surface.clone()
    }
}
