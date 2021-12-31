use crate::gfx::scan_buffer_view_api::IScanBufferView;

pub struct ScanBufferViewWgpu {
    _frame: wgpu::SurfaceTexture,
    _format: wgpu::TextureFormat,
}

impl ScanBufferViewWgpu {
    pub fn new(frame: wgpu::SurfaceTexture, format: wgpu::TextureFormat) -> Self {
        Self {
            _frame: frame,
            _format: format,
        }
    }

    pub fn get_frame(&self) -> &wgpu::SurfaceTexture {
        &self._frame
    }

    pub fn move_frame(self) -> wgpu::SurfaceTexture {
        self._frame
    }

    pub fn get_format(&self) -> wgpu::TextureFormat {
        self._format.clone()
    }

    pub fn create_view(&self) -> wgpu::TextureView {
        self._frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    }
}

impl IScanBufferView for ScanBufferViewWgpu {}
