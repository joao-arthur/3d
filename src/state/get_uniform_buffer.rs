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
        contents: bytes_of(&Uniforms {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            camera_position: [0.0, 0.0, 4.0, 1.0],
            light_direction: [-0.5, 0.8, 0.4, 0.0],
            light_color: [1.0, 0.92, 0.78, 1.0],
            ambient_color: [0.12, 0.15, 0.22, 1.0],
            light_params: [0.35, 32.0, 0.0, 0.0],
        }),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    })
}
