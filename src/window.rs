use std::sync::Arc;

use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::Window,
};

pub struct WindowState {
    window: Arc<Window>,
}

#[derive(Default)]
pub struct AppWindow {
    state: Option<WindowState>,
}

impl ApplicationHandler for AppWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }
        let attributes = Window::default_attributes().with_title("wgpu GLB Tree");
        let window =
            Arc::new(event_loop.create_window(attributes).expect("failed to create window"));
        let state = WindowState { window };
        self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(_state) = &mut self.state else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(_size) => {}
            WindowEvent::RedrawRequested => {}
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.state {
            state.window.request_redraw();
        }
    }
}
