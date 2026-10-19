use winit::{
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    adapter::get_wgpu_adapter, device::get_wgpu_device, instance::get_wgpu_instance,
    window::get_winit_window,
};

mod adapter;
mod device;
mod instance;
mod window;

#[derive(Debug, PartialEq, Default)]
struct App {}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = get_winit_window(&event_loop).map_err(|_| ()).unwrap();
        let instance = get_wgpu_instance();
        let size = window.inner_size();
        let surface = instance.create_surface(window).unwrap();
        let adapter = pollster::block_on(get_wgpu_adapter(&instance, &surface)).unwrap();
        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(|format| format.is_srgb())
            .unwrap_or(capabilities.formats[0]);
        let config = surface.get_default_config(&adapter, size.width, size.height).unwrap();
        let (device, queue) = pollster::block_on(get_wgpu_device(&adapter)).unwrap();
        surface.configure(&device, &config);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
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
    let mut app = App::default();
    event_loop.run_app(&mut app).map_err(|_| ())?;
    Ok(())
}
