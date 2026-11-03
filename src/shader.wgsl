
struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    normal_matrix: mat4x4<f32>,

    camera_position: vec4<f32>,
    light_direction: vec4<f32>,
    light_color: vec4<f32>,
    ambient_color: vec4<f32>,
    light_params: vec4<f32>,
};

@group(0)
@binding(0)
var<uniform> uniforms: Uniforms;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,

    @location(0) color: vec4<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) normal: vec3<f32>,
};


@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let world_position =
        uniforms.model * vec4<f32>(input.position, 1.0);

    let world_normal =
        uniforms.normal_matrix * vec4<f32>(input.normal, 0.0);

    output.clip_position =
        uniforms.view_proj * world_position;

    output.world_position = world_position.xyz;
    output.normal = normalize(world_normal.xyz);
    output.color = input.color;

    return output;
}


@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(input.normal);

    // Direction toward the light.
    let light_dir = normalize(uniforms.light_direction.xyz);

    // Direction toward the camera.
    let view_dir = normalize(
        uniforms.camera_position.xyz - input.world_position
    );

    // Blinn-Phong halfway vector.
    let halfway_dir = normalize(light_dir + view_dir);

    // Diffuse component.
    let diffuse_factor = max(dot(normal, light_dir), 0.0);

    let diffuse =
        uniforms.light_color.rgb *
        diffuse_factor *
        input.color.rgb;

    // Specular component.
    let specular_factor = pow(
        max(dot(normal, halfway_dir), 0.0),
        uniforms.light_params.y
    );

    let specular =
        uniforms.light_color.rgb *
        specular_factor *
        uniforms.light_params.x;

    // Ambient component.
    let ambient =
        uniforms.ambient_color.rgb *
        input.color.rgb;

    // Final illumination.
    let result = ambient + diffuse + specular;

    return vec4<f32>(result, input.color.a);
}
