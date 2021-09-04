use super::VkAutoCommandBufferBuilder;

pub struct DispatchCommandBuilder {
    _gropup_count_x: u32,
    _gropup_count_y: u32,
    _gropup_count_z: u32,
}

impl DispatchCommandBuilder {
    pub fn new(group_count_x: u32, group_count_y: u32, group_count_z: u32) -> Self {
        Self {
            _gropup_count_x: group_count_x,
            _gropup_count_y: group_count_y,
            _gropup_count_z: group_count_z,
        }
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        command_builder
        // command_builder.dispatch([
        //     self._gropup_count_x,
        //     self._gropup_count_y,
        //     self._gropup_count_z,
        // ])
    }
}
