use glam::{Mat4, Vec3};
use wgpu::CurrentSurfaceTexture;

use crate::{data::uniforms::Uniforms, state::state::State};

pub fn app_render(state: &mut State) {
    let (frame, reconfigure_after_present) = match state.surface.get_current_texture() {
        CurrentSurfaceTexture::Success(frame) => (frame, false),
        CurrentSurfaceTexture::Suboptimal(frame) => (frame, true),
        CurrentSurfaceTexture::Lost | CurrentSurfaceTexture::Outdated => {
            state.surface.configure(&state.device, &state.surface_config);
            return;
        }
        CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => {
            return;
        }
        CurrentSurfaceTexture::Validation => {
            eprintln!("Surface acquisition validation error");
            return;
        }
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let eye = Vec3::new(0.0, 0.7, 4.0);
    let target = Vec3::ZERO;
    let up = Vec3::Y;
    let view_matrix = glam::camera::rh::view::look_at_mat4(eye, target, up);
    let aspect = state.surface_config.width as f32 / state.surface_config.height as f32;
    let projection_matrix =
        glam::camera::rh::proj::directx::perspective(45.0_f32.to_radians(), aspect, 0.1, 100.0);
    let model_matrix = Mat4::IDENTITY;
    let mvp = projection_matrix * view_matrix * model_matrix;
    state.queue.write_buffer(
        &state.uniform_buffer,
        0,
        bytemuck::bytes_of(&Uniforms { mvp: mvp.to_cols_array_2d() }),
    );
    let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Tree Render Encoder"),
    });
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Tree Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.08, g: 0.10, b: 0.14, a: 1.0 }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &state.texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&state.render_pipeline);
        pass.set_bind_group(0, &state.uniform_bind_group, &[]);
        pass.set_vertex_buffer(0, state.model.vertex_buffer.slice(..));
        pass.set_index_buffer(state.model.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..state.model.index_count, 0, 0..1);
    }
    state.queue.submit([encoder.finish()]);
    state.queue.present(frame);
    if reconfigure_after_present {
        state.surface.configure(&state.device, &state.surface_config);
    }
    state.window.request_redraw();
}
