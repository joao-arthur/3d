use wgpu::{
    Adapter, Instance, PowerPreference, RequestAdapterError, RequestAdapterOptions, Surface,
};

pub fn get_wgpu_adapter(instance: &Instance, surface: &Surface) -> impl Future<Output = Result<Adapter, RequestAdapterError>> {
    instance.request_adapter(&RequestAdapterOptions {
        power_preference: if cfg!(debug_assertions) {
            PowerPreference::LowPower
        } else {
            PowerPreference::HighPerformance
        },
        compatible_surface: Some(surface),
        force_fallback_adapter: false,
        apply_limit_buckets: true,
    })
}
