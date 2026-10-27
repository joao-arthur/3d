
use winit::{
    event_loop::{ControlFlow, EventLoop},
};

use crate::app::App;

mod adapter;
mod command_encoder;
mod device;
mod instance;
mod pipeline_layout;
mod pipeline_render;
mod render_pass;
mod shader_module;
mod window;
mod load_model;
mod model;
mod vertex;
mod app;

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();
    let event_loop = EventLoop::new().map_err(|_| ())?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).map_err(|_| ())?;
    Ok(())
}
