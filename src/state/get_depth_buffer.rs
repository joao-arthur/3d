use wgpu::{Device, Texture, TextureView, TextureViewDescriptor};

use super::texture::get_wgpu_texture;

pub fn get_depth_buffer(device: &Device, width: u32, height: u32) -> (Texture, TextureView) {
    let texture = get_wgpu_texture(&device, width, height);
    let texture_view = texture.create_view(&TextureViewDescriptor::default());
    (texture, texture_view)
}
