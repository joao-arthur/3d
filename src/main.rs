use crate::{adapter::get_wgpu_adapter, instance::get_wgpu_instance};

mod adapter;
mod instance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance = get_wgpu_instance();
    // let adapter = get_wgpu_adapter(&instance, surface);

    Ok(())
}
