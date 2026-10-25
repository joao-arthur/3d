use wgpu::Buffer;

pub struct Model {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}
