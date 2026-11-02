use wgpu::{BindGroupLayout, Device, PipelineLayout, PipelineLayoutDescriptor};

pub fn get_wgpu_pipeline_layout(
    device: &Device,
    uniform_bind_group_layout: &BindGroupLayout,
) -> PipelineLayout {
    device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Tree Pipeline Layout"),
        bind_group_layouts: &[Some(uniform_bind_group_layout)],
        immediate_size: 0,
    })
}
