use bytemuck::bytes_of;
use glam::Mat4;
use wgpu::{
    Buffer, BufferUsages, Device,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::data::uniforms::Uniforms;

pub fn get_uniform_buffer(device: &Device) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Camera Uniform Buffer"),
        contents: bytes_of(&Uniforms { mvp: Mat4::IDENTITY.to_cols_array_2d() }),
        usage: BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}
