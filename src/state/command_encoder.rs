use wgpu::{CommandEncoder, CommandEncoderDescriptor, Device};

pub fn get_wgpu_command_encoder(device: &Device) -> CommandEncoder {
    device.create_command_encoder(&CommandEncoderDescriptor { label: Some("Tree Render Encoder") })
}
