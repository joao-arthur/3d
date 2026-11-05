use std::sync::Arc;

use wgpu::{
    BindGroup, Buffer, Device, Instance, Queue, RenderPipeline, Surface, SurfaceConfiguration,
    Texture, TextureView,
};
use winit::window::Window;

use crate::data::model::Model;

pub struct State {
    pub window: Arc<Window>,
    pub instance: Instance,
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub render_pipeline: RenderPipeline,
    pub model: Model,
    pub uniform_buffer: Buffer,
    pub uniform_bind_group: BindGroup,
    pub texture: Texture,
    pub texture_view: TextureView,
    // pipeline_render: Option<RenderPipeline>,
    // vertex_buffer: Option<Buffer>,
    pub instance_buffer: Buffer,
    pub instances: Vec<crate::data::instance::Instance>,
}
