use wgpu::Buffer;

pub struct Model {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_count: u32,
}
