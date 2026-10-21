use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Device, FragmentState, PipelineCompilationOptions,
    PipelineLayout, RenderPipeline, RenderPipelineDescriptor, ShaderModule, SurfaceConfiguration,
    VertexState,
};

pub fn get_wgpu_pipeline_render(
    device: &Device,
    pipeline_layout: &PipelineLayout,
    surface_config: &SurfaceConfiguration,
    shader_module: &ShaderModule,
) -> RenderPipeline {
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: PipelineCompilationOptions::default(),
        },
        fragment: Some(FragmentState {
            module: &shader_module,
            entry_point: Some("fs_main"),
            targets: &[Some(ColorTargetState {
                format: surface_config.format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
            compilation_options: PipelineCompilationOptions::default(),
        }),
        cache,
        depth_stencil,
        multisample,
        multiview_mask,
        primitive, // continued ...
    })
}
