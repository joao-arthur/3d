use std::sync::Arc;

use winit::event_loop::ActiveEventLoop;

use crate::state::{
    get_depth_buffer::get_depth_buffer,
    get_model::get_model,
    get_uniform_bind_group::get_uniform_bind_group,
    get_uniform_bind_group_layout::get_uniform_bind_group_layout,
    get_uniform_buffer::get_uniform_buffer,
    pipeline_layout::get_wgpu_pipeline_layout,
    render_pipeline::get_wgpu_render_pipeline,
    shader_module::get_wgpu_shader_module,
    state::State,
};

use super::{
    adapter::get_wgpu_adapter, device::get_wgpu_device, instance::get_wgpu_instance,
    surface_config::get_wgpu_surface_config, texture_format::get_wgpu_texture_format,
    window::get_winit_window,
};

pub enum SetupError {
    CreateWindow,
    CreateSurface,
    CreateAdapter,
    CreateDevice,
    CreateSurfaceConfig,
}

pub async fn state_setup(event_loop: &ActiveEventLoop) -> Result<State, SetupError> {
    let window = Arc::new(get_winit_window(event_loop).map_err(|_| SetupError::CreateWindow)?);
    let instance = get_wgpu_instance();
    let size = window.inner_size();
    let surface = instance.create_surface(window.clone()).map_err(|_| SetupError::CreateSurface)?;
    let adapter =
        get_wgpu_adapter(&instance, &surface).await.map_err(|_| SetupError::CreateAdapter)?;
    let (device, queue) = get_wgpu_device(&adapter).await.map_err(|_| SetupError::CreateDevice)?;
    let capabilities = surface.get_capabilities(&adapter);
    let texture_format = get_wgpu_texture_format(&capabilities);
    let surface_config = get_wgpu_surface_config(
        &surface,
        &adapter,
        (size.width, size.height),
        texture_format,
        capabilities.alpha_modes[0],
    )
    .ok_or(SetupError::CreateSurfaceConfig)?;
    surface.configure(&device, &surface_config);
    let model = get_model(&device);
    let uniform_buffer = get_uniform_buffer(&device);
    let uniform_bind_group_layout = get_uniform_bind_group_layout(&device);
    let uniform_bind_group =
        get_uniform_bind_group(&device, &uniform_buffer, &uniform_bind_group_layout);
    let shader_module = get_wgpu_shader_module(&device);
    let pipeline_layout = get_wgpu_pipeline_layout(&device, &uniform_bind_group_layout);
    let render_pipeline =
        get_wgpu_render_pipeline(&device, &pipeline_layout, &shader_module, &texture_format);
    let (texture, texture_view) = get_depth_buffer(&device, size.width, size.height);

    Ok(State {
        window,
        instance,
        device,
        queue,
        surface,
        surface_config,
        render_pipeline,
        model,
        uniform_buffer,
        uniform_bind_group,
        texture,
        texture_view,
    })
}
