use crate::instance::get_wgpu_instance;

mod instance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance = get_wgpu_instance();

    Ok(())
}
