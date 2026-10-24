use wgpu::{CommandEncoder, Device};

pub fn get_wgpl_command_encoder<'a>(device: &Device) -> CommandEncoder {
    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") })
}
