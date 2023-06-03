use sjgfx_interface::ShaderStage;
use sjgfx_util::ShaderReflection;

pub fn create_bind_group_layout(
    device: &wgpu::Device,
    vertex_shader_binary: &[u8],
    pixel_shader_binary: &[u8],
) -> Option<wgpu::BindGroupLayout> {
    let shader_reflection_vertex = ShaderReflection::new_from_biinary(vertex_shader_binary);
    let shader_reflection_pixel = ShaderReflection::new_from_biinary(pixel_shader_binary);

    let entries = {
        let mut vertex_entries =
            create_bind_group_layout_entries(&shader_reflection_vertex, &ShaderStage::Vertex);
        let mut pixel_entries =
            create_bind_group_layout_entries(&shader_reflection_pixel, &ShaderStage::Pixel);
        vertex_entries.append(&mut pixel_entries);
        vertex_entries
    };
    if entries.is_empty() {
        None
    } else {
        Some(
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &entries,
            }),
        )
    }
}

pub fn create_pipeline_layout(
    device: &wgpu::Device,
    vertex_shader_binary: &[u8],
    pixel_shader_binary: &[u8],
) -> wgpu::PipelineLayout {
    let bind_group_layout =
        create_bind_group_layout(device, vertex_shader_binary, pixel_shader_binary);
    if let Some(bind_group_layout) = bind_group_layout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        })
    } else {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        })
    }
}

pub fn create_bind_group_layout_entries(
    shader_reflection: &ShaderReflection,
    shader_stage: &ShaderStage,
) -> Vec<wgpu::BindGroupLayoutEntry> {
    let mut uniform_buffer_enetries = shader_reflection
        .uniform_buffers()
        .iter()
        .map(|x| wgpu::BindGroupLayoutEntry {
            binding: x.binding as u32,
            visibility: crate::util::convert_shader_stage(shader_stage.clone()),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new(x.size as u64),
            },
            count: None,
        })
        .collect::<Vec<wgpu::BindGroupLayoutEntry>>()
        .to_vec();

    let mut shader_storage_buffer_enetries = shader_reflection
        .shader_storage_buffer()
        .iter()
        .map(|x| wgpu::BindGroupLayoutEntry {
            binding: x.binding as u32,
            visibility: crate::util::convert_shader_stage(shader_stage.clone()),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
        .collect::<Vec<wgpu::BindGroupLayoutEntry>>()
        .to_vec();

    let mut entries = Vec::new();
    entries.append(&mut uniform_buffer_enetries);
    entries.append(&mut shader_storage_buffer_enetries);
    entries
}

pub fn create_vertex_attributes(
    shader_reflection: &ShaderReflection,
) -> Vec<wgpu::VertexAttribute> {
    let vertex_attributes = shader_reflection
        .entry_point
        .attribures()
        .iter()
        .map(|attribute| wgpu::VertexAttribute {
            format: crate::util::convert_attribute_format(attribute.format()),
            offset: attribute.offset() as u64,
            shader_location: attribute.location(),
        })
        .collect::<Vec<wgpu::VertexAttribute>>()
        .to_vec();

    vertex_attributes
}
