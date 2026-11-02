use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::app::App;

mod app;
mod state;
mod data;

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();
    let event_loop = EventLoop::new().map_err(|_| ())?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).map_err(|_| ())?;
    Ok(())
}
