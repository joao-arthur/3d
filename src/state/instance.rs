use wgpu::{
    BackendOptions, Backends, Instance, InstanceDescriptor, InstanceFlags, MemoryBudgetThresholds,
};

pub fn get_wgpu_instance() -> Instance {
    Instance::new(InstanceDescriptor {
        backends: Backends::default(),
        flags: if cfg!(debug_assertions) {
            InstanceFlags::DEBUG
                | InstanceFlags::VALIDATION
                | InstanceFlags::VALIDATION_INDIRECT_CALL
        } else {
            InstanceFlags::empty()
        },
        memory_budget_thresholds: MemoryBudgetThresholds::default(),
        backend_options: BackendOptions::default(),
        display: None,
    })
}
