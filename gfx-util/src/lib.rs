mod shader_compiler;
pub use shader_compiler::{ShaderCompiler, ShaderStage};

use sjgfx_interface::{BufferInfo, GpuAccess, IBuffer, IDevice};

pub struct ObjData<TBuffer: IBuffer> {
    pub vertex_buffer: TBuffer,
    pub index_buffer: TBuffer,
    pub vertex_count: i32,
    pub index_count: i32,
}

pub fn load_obj<TDevice, TBuffer>(device: &mut TDevice, obj_text: &str) -> ObjData<TBuffer>
where
    TDevice: IDevice,
    TBuffer: IBuffer<DeviceType = TDevice>,
{
    let mut buffer = obj_text.as_bytes();
    let result = tobj::load_obj_buf(
        &mut buffer,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
        |_| {
            let mut material_buffer: &[u8] = &[];
            tobj::load_mtl_buf(&mut material_buffer)
        },
    )
    .unwrap();

    let model = result.0.first().unwrap();
    let vertex_count = model.mesh.positions.len() / 3;
    let index_count = model.mesh.indices.len();

    // Buffer に書き込む
    let vertex_buffer = TBuffer::new(
        device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(std::mem::size_of::<f32>() * 6 * vertex_count),
    );
    let index_buffer = TBuffer::new(
        device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::INDEX_BUFFER)
            .set_size(std::mem::size_of::<u32>() * index_count),
    );

    vertex_buffer.map_as_slice_mut(|x: &mut [f32]| {
        let mut current_index = 0;

        for index in 0..vertex_count {
            x[current_index] = model.mesh.positions[3 * index + 0];
            current_index += 1;

            x[current_index] = model.mesh.positions[3 * index + 1];
            current_index += 1;

            x[current_index] = model.mesh.positions[3 * index + 2];
            current_index += 1;

            x[current_index] = model.mesh.normals[3 * index + 0];
            current_index += 1;

            x[current_index] = model.mesh.normals[3 * index + 1];
            current_index += 1;

            x[current_index] = model.mesh.normals[3 * index + 2];
            current_index += 1;
        }
    });

    index_buffer.map_as_slice_mut(|x: &mut [u32]| {
        for index in 0..index_count {
            x[index] = model.mesh.indices[index];
        }
    });

    ObjData {
        vertex_buffer,
        index_buffer,
        vertex_count: vertex_count as i32,
        index_count: index_count as i32,
    }
}
