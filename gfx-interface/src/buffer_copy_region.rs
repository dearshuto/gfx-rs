#[derive(Debug, Default)]
pub struct BufferCopyRegion {
    src_offset: isize,
    dst_offset: isize,
    copy_size: usize,
}

impl BufferCopyRegion {
    pub fn get_src_offset(&self) -> isize {
        self.src_offset
    }

    pub fn set_src_offset(mut self, offset: isize) -> Self {
        self.src_offset = offset;
        self
    }

    pub fn get_dst_offset(&self) -> isize {
        self.dst_offset
    }

    pub fn set_dst_offset(mut self, offset: isize) -> Self {
        self.dst_offset = offset;
        self
    }

    pub fn get_copy_size(&self) -> usize {
        self.copy_size
    }

    pub fn set_copy_size(mut self, size: usize) -> Self {
        self.copy_size = size;
        self
    }
}
