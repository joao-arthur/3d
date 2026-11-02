use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource};

pub fn get_wgpu_shader_module(device: &Device) -> ShaderModule {
    device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Tree Shader"),
        source: ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
    })
}
