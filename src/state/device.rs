use wgpu::{
    Adapter, Device, DeviceDescriptor, ExperimentalFeatures, Features, Label, Limits, MemoryHints,
    Queue, RequestDeviceError, Trace,
};

pub fn get_wgpu_device(
    adapter: &Adapter,
) -> impl Future<Output = Result<(Device, Queue), RequestDeviceError>> {
    adapter.request_device(&DeviceDescriptor {
        label: Label::Some("Device"),
        required_features: Features::empty(),
        experimental_features: ExperimentalFeatures::disabled(),
        required_limits: Limits::default(),
        memory_hints: MemoryHints::default(),
        trace: Trace::Off,
    })
}
