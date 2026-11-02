use wgpu::{
    Color, CommandEncoder, LoadOp, Operations, RenderPass, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp, TextureView,
};

pub fn get_wpgu_render_pass<'a>(
    command_encoder: &'a mut CommandEncoder,
    frame_texture_view: &TextureView,
    state_texture_view: &TextureView,
) -> RenderPass<'a> {
    command_encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("Tree Render Pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: frame_texture_view,
            depth_slice: None,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color { r: 0.08, g: 0.10, b: 0.14, a: 1.0 }),
                store: StoreOp::Store,
            },
        })],
        depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
            view: state_texture_view,
            depth_ops: Some(Operations { load: LoadOp::Clear(1.0), store: StoreOp::Store }),
            stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    })
}
