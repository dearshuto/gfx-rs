use crate::gfx::scan_buffer_view_api::IScanBufferView;

pub struct ScanBufferViewWgpu {
    _frame: wgpu::SurfaceFrame,
}

impl ScanBufferViewWgpu {
    pub fn new(frame: wgpu::SurfaceFrame) -> Self {
        Self { _frame: frame }
    }

    pub fn get_frame(&self) -> &wgpu::SurfaceFrame {
        &self._frame
    }

    pub fn move_frame(self) -> wgpu::SurfaceFrame {
        self._frame
    }

    pub fn create_view(&self) -> wgpu::TextureView {
        self._frame
            .output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    }
}

impl IScanBufferView for ScanBufferViewWgpu {}
