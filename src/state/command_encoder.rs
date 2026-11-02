use wgpu::{CommandEncoder, Device};

pub fn get_wgpu_command_encoder(device: &Device) -> CommandEncoder {
    device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Tree Render Encoder"),
    })
}
