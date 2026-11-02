use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, Device};

pub fn get_uniform_bind_group(
    device: &Device,
    uniform_buffer: &Buffer,
    uniform_bind_group_layout: &BindGroupLayout,
) -> BindGroup {
    device.create_bind_group(&BindGroupDescriptor {
        label: Some("Camera Bind Group"),
        layout: uniform_bind_group_layout,
        entries: &[BindGroupEntry { binding: 0, resource: uniform_buffer.as_entire_binding() }],
    })
}
