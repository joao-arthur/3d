#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions::default()
    ).await.expect("Failed to create adapter");
    let downlevel_capabilities = adapter.get_downlevel_capabilities();
    println!("Adapter: {:#?}", adapter.get_info());
    println!("Capacities: {:#?}", downlevel_capabilities);
    if !downlevel_capabilities
        .flags
        .contains(wgpu::DownlevelFlags::COMPUTE_SHADERS)
    {
        panic!("Adapter does not support compute shaders");
    }
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::MemoryUsage,
            trace: wgpu::Trace::Off,
        }
    ).await.expect("Failed to create device");

    Ok(())
}
