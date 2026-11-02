use crate::state::{get_depth_buffer::get_depth_buffer, state::State};

pub fn app_resize(state: &mut State, width: u32, height: u32) {
    if width == 0 || height == 0 {
        return;
    }
    state.surface_config.width = width;
    state.surface_config.height = height;
    state.surface.configure(&state.device, &state.surface_config);
    let (texture, texture_view) = get_depth_buffer(&state.device, width, height);
    state.texture = texture;
    state.texture_view = texture_view;
}
