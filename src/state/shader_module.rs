use wgpu::{Device, ShaderModule};

pub fn get_wgpu_shader_module(device: &Device) -> ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Tree Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
    })
}
