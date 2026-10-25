use wgpu::{
    Color, CommandEncoder, LoadOp, Operations, RenderPass, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, TextureView,
};

pub fn get_wgpl_render_pass_descriptor<'a>(
    command_encoder: &'a mut CommandEncoder,
    texture_view: &TextureView,
) -> RenderPass<'a> {
    command_encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: texture_view,
            resolve_target: None,
            depth_slice: None,
            ops: Operations {
                load: LoadOp::Clear(Color { r: 0.2, g: 0.4, b: 0.7, a: 1.0 }),
                store: StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
        multiview_mask: None,
    })
}
