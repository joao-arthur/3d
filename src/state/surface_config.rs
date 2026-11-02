use wgpu::{Adapter, CompositeAlphaMode, Surface, SurfaceConfiguration, TextureFormat};

pub fn get_wgpu_surface_config(
    surface: &Surface,
    adapter: &Adapter,
    (width, height): (u32, u32),
    texture_format: TextureFormat,
    alpha_mode: CompositeAlphaMode
) -> Option<SurfaceConfiguration> {
    let mut surface_config = surface.get_default_config(&adapter, width, height)?;
    surface_config.format = texture_format;
    surface_config.present_mode = wgpu::PresentMode::Fifo;
    surface_config.alpha_mode = alpha_mode;
    Some(surface_config)
}
