use wgpu::{
    Buffer, Device, Instance, Queue, RenderPipeline, Surface, SurfaceConfiguration,
    TextureViewDescriptor,
};
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::{
    adapter::get_wgpu_adapter, command_encoder::get_wgpl_command_encoder, device::get_wgpu_device,
    instance::get_wgpu_instance, pipeline_layout::get_wgpu_pipeline_layout,
    pipeline_render::get_wgpu_pipeline_render, render_pass::get_wgpl_render_pass_descriptor,
    shader_module::get_wgpu_shader_module, window::get_winit_window,
};

#[derive(Default)]
pub struct App<'a> {
    instance: Option<Instance>,
    device: Option<Device>,
    queue: Option<Queue>,
    surface_config: Option<SurfaceConfiguration>,
    surface: Option<Surface<'a>>,
    pipeline_render: Option<RenderPipeline>,
    vertex_buffer: Option<Buffer>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // let window = get_winit_window(event_loop).map_err(|_| ()).unwrap();
        // let instance = get_wgpu_instance();
        // let size = window.inner_size();
        // let surface = instance.create_surface(window).unwrap();
        // let adapter = pollster::block_on(get_wgpu_adapter(&instance, &surface)).unwrap();
        // let surface_config = surface.get_default_config(&adapter, size.width, size.height).unwrap();
        // let (device, queue) = pollster::block_on(get_wgpu_device(&adapter)).unwrap();
        surface.configure(&device, &surface_config);
        let shader_module = get_wgpu_shader_module(&device);
        let pipeline_layout = get_wgpu_pipeline_layout(&device);
        let pipeline_render =
            get_wgpu_pipeline_render(&device, &pipeline_layout, &surface_config, &shader_module);
        self.instance = Some(instance);
        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);
        self.surface_config = Some(surface_config);
        self.pipeline_render = Some(pipeline_render);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if size.width == 0 || size.height == 0 {
                    return;
                }
                let Some(surface) = &self.surface else {
                    return;
                };
                let Some(device) = &self.device else {
                    return;
                };
                let Some(surface_config) = &mut self.surface_config else {
                    return;
                };
                surface_config.width = size.width;
                surface_config.height = size.height;
                surface.configure(device, surface_config);
            }
            WindowEvent::RedrawRequested => {
                let Some(surface) = &self.surface else {
                    return;
                };
                let Some(device) = &self.device else {
                    return;
                };
                let Some(queue) = &self.queue else {
                    return;
                };
                let Some(pipeline_render) = &self.pipeline_render else {
                    return;
                };
                let surface_texture_option = match surface.get_current_texture() {
                    wgpu::CurrentSurfaceTexture::Success(surface_texture) => Some(surface_texture),
                    wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                        Some(surface_texture)
                    }
                    wgpu::CurrentSurfaceTexture::Timeout
                    | wgpu::CurrentSurfaceTexture::Occluded
                    | wgpu::CurrentSurfaceTexture::Validation => None,
                    wgpu::CurrentSurfaceTexture::Outdated => None,
                    wgpu::CurrentSurfaceTexture::Lost => None,
                };
                let Some(surface_texture) = surface_texture_option else {
                    return;
                };
                let mut command_encoder = get_wgpl_command_encoder(device);
                {
                    let texture_view =
                        surface_texture.texture.create_view(&TextureViewDescriptor::default());
                    let mut render_pass =
                        get_wgpl_render_pass_descriptor(&mut command_encoder, &texture_view);
                    render_pass.set_pipeline(&pipeline_render);
                    render_pass.draw(0..3, 0..1);
                }
                queue.submit(Some(command_encoder.finish()));
                queue.present(surface_texture);
            }
            _ => {}
        }
    }
}
