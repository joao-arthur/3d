use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::{app::{render::app_render, resize::app_resize}, state::{setup::{SetupError, state_setup}, state::State}};

#[derive(Default)]
pub struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }
        let result = pollster::block_on(state_setup(event_loop));
        match result {
            Ok(app_state) => {
                self.state = Some(app_state);
            },
            Err(state_setup_err) => {
                match state_setup_err {
                    SetupError::CreateWindow => {
                        println!("CreateWindow");
                    }
                    SetupError::CreateSurface => {
                        println!("CreateSurface");
                    }
                    SetupError::CreateAdapter => {
                        println!("CreateAdapter");
                    }
                    SetupError::CreateDevice => {
                        println!("CreateDevice");
                    }
                    SetupError::CreateSurfaceConfig => {
                        println!("CreateSurfaceConfig");
                    }
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = self.state.as_mut() else {
            return;
        };
        if state.window.id() != window_id {
            return;
        }
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                app_resize(state, size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                app_render(state);
            }
            _ => {}
        }
    }
}
