use std::cell::RefCell;

use wgpu::{Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::{
    adapter::get_wgpu_adapter, command_encoder::get_wgpl_command_encoder, device::get_wgpu_device,
    instance::get_wgpu_instance, pipeline_layout::get_wgpu_pipeline_layout,
    pipeline_render::get_wgpu_pipeline_render, render_pass::get_wgpl_render_pass_descriptor,
    shader_module::get_wgpu_shader_module, window::get_winit_window,
};

mod adapter;
mod command_encoder;
mod device;
mod instance;
mod pipeline_layout;
mod pipeline_render;
mod render_pass;
mod shader_module;
mod window;

struct App<'a> {
    // window: Option<Window>,
    instance: Option<Instance>,
    device: Option<Device>,
    queue: Option<Queue>,
    surface_config: Option<SurfaceConfiguration>,
    surface: Option<Surface<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = get_winit_window(event_loop).map_err(|_| ()).unwrap();
        let instance = get_wgpu_instance();
        let size = window.inner_size();
        let surface = instance.create_surface(window).unwrap();
        let adapter = pollster::block_on(get_wgpu_adapter(&instance, &surface)).unwrap();
        // let capabilities = surface.get_capabilities(&adapter);
        let surface_config = surface.get_default_config(&adapter, size.width, size.height).unwrap();
        let (device, queue) = pollster::block_on(get_wgpu_device(&adapter)).unwrap();
        surface.configure(&device, &surface_config);

        // self.window = Some(window);
        self.instance = Some(instance);
        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);
        self.surface_config = Some(surface_config);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(surface) = &self.surface else {
            return;
        };
        let Some(device) = &self.device else {
            return;
        };
        let Some(surface_config) = &self.surface_config else {
            return;
        };
        let Some(queue) = &self.queue else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(_size) => {}
            WindowEvent::RedrawRequested => {
                let output_option = match surface.get_current_texture() {
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
                let Some(output) = output_option else {
                    return;
                };
                let mut command_encoder = get_wgpl_command_encoder(device);
                {
                    let shader_module = get_wgpu_shader_module(device);
                    let pipeline_layout = get_wgpu_pipeline_layout(device);
                    let pipeline_render = get_wgpu_pipeline_render(
                        device,
                        &pipeline_layout,
                        surface_config,
                        &shader_module,
                    );
                    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                    let mut render_pass =
                        get_wgpl_render_pass_descriptor(&mut command_encoder, &view);
                    render_pass.set_pipeline(&pipeline_render);
                    render_pass.draw(0..3, 0..1);
                }
                queue.submit(Some(command_encoder.finish()));
                queue.present(output);
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();
    let event_loop = EventLoop::new().map_err(|_| ())?;
    if cfg!(debug_assertions) {
        event_loop.set_control_flow(ControlFlow::Wait);
    } else {
        event_loop.set_control_flow(ControlFlow::Poll);
    }
    let mut app = App {
        // window: None,
        instance: None,
        device: None,
        queue: None,
        surface: None,
        surface_config: None,
    };
    event_loop.run_app(&mut app).map_err(|_| ())?;
    Ok(())
}
