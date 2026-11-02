use wgpu::{SurfaceCapabilities, TextureFormat};

pub fn get_wgpu_texture_format(capabilities: &SurfaceCapabilities) -> TextureFormat {
    capabilities.formats.iter().copied().find(|f| f.is_srgb()).unwrap_or(capabilities.formats[0])
}
