use wgpu::{Device, PipelineLayout};

pub fn get_wgpu_pipeline_layout(device: &Device) -> PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        immediate_size: 0,
    })
}
