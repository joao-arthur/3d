use std::sync::atomic::{AtomicUsize, Ordering};

use bytemuck::bytes_of;
use glam::{Mat4, Vec3, camera};
use wgpu::{CurrentSurfaceTexture, IndexFormat, TextureViewDescriptor};

use crate::{
    data::uniforms::Uniforms,
    state::{
        command_encoder::get_wgpu_command_encoder, render_pass::get_wpgu_render_pass, state::State,
    },
};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn app_render(state: &mut State) {
    let tick = COUNTER.fetch_add(1, Ordering::Relaxed);
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
    let frame_view = frame.texture.create_view(&TextureViewDescriptor::default());
    let eye = Vec3::new(tick as f32 / 100f32, tick as f32 / 100f32, tick as f32 / 100f32);
    let target = Vec3::ZERO;
    let up = Vec3::Y;
    let view_matrix = camera::rh::view::look_at_mat4(eye, target, up);
    let aspect = state.surface_config.width as f32 / state.surface_config.height as f32;
    let projection_matrix =
        camera::rh::proj::directx::perspective(45.0_f32.to_radians(), aspect, 0.1, 100.0);
    let model_matrix = Mat4::IDENTITY;
    state.queue.write_buffer(
        &state.uniform_buffer,
        0,
        bytes_of(&Uniforms {
            view_proj: (projection_matrix * view_matrix).to_cols_array_2d(),
            camera_position: [eye.x, eye.y, eye.z, 1.0],
            light_direction: [-0.5, 0.8, 0.4, 0.0],
            light_color: [1.0, 0.92, 0.78, 1.0],
            ambient_color: [0.12, 0.15, 0.22, 1.0],
            light_params: [0.35, 32.0, 0.0, 0.0],
        }),
    );
    let mut command_encoder = get_wgpu_command_encoder(&state.device);
    {
        let mut render_pass =
            get_wpgu_render_pass(&mut command_encoder, &frame_view, &state.texture_view);
        render_pass.set_pipeline(&state.render_pipeline);
        render_pass.set_bind_group(0, &state.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, state.model.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, state.instance_buffer.slice(..));
        render_pass.set_index_buffer(state.model.index_buffer.slice(..), IndexFormat::Uint32);
        render_pass.draw_indexed(0..state.model.index_count, 0, 0..state.instances.len() as u32);
    }
    state.queue.submit([command_encoder.finish()]);
    state.queue.present(frame);
    if reconfigure_after_present {
        state.surface.configure(&state.device, &state.surface_config);
    }
    state.window.request_redraw();
}
