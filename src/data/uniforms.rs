use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
    pub model: [[f32; 4]; 4],
    pub normal_matrix: [[f32; 4]; 4],
    pub camera_position: [f32; 4],
    pub light_direction: [f32; 4], // Direction FROM the surface TOWARD the light.
    pub light_color: [f32; 4],
    pub ambient_color: [f32; 4],
    pub light_params: [f32; 4], // x = specular strength, y = shininess
}
