use crate::{adapter::get_wgpu_adapter, device::get_wgpu_device, instance::get_wgpu_instance};

mod adapter;
mod device;
mod instance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let instance = get_wgpu_instance();
    // instance.create_surface(target);
    // let adapter = get_wgpu_adapter(&instance, surface);
    // let (device, queue) = get_wgpu_device(&adapter);
    Ok(())
}
