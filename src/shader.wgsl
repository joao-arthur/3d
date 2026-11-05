struct Uniforms {
    view_proj: mat4x4<f32>,
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
    @location(3) model_0: vec4<f32>,
    @location(4) model_1: vec4<f32>,
    @location(5) model_2: vec4<f32>,
    @location(6) model_3: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) normal: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let model = mat4x4<f32>(input.model_0, input.model_1, input.model_2, input.model_3);
    var output: VertexOutput;
    let world_position = model * vec4<f32>(input.position, 1.0);
    let world_normal = (model * vec4<f32>(input.normal, 0.0)).xyz;
    output.clip_position = uniforms.view_proj * world_position;
    output.world_position = world_position.xyz;
    output.normal = normalize(world_normal);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(input.normal);
    let light_dir = normalize(uniforms.light_direction.xyz);
    let view_dir = normalize(uniforms.camera_position.xyz - input.world_position);
    let halfway_dir = normalize(light_dir + view_dir);
    let diffuse_factor = max(dot(normal, light_dir), 0.0);
    let diffuse = uniforms.light_color.rgb * diffuse_factor * input.color.rgb;
    let specular_factor = pow(max(dot(normal, halfway_dir), 0.0), uniforms.light_params.y);
    let specular = uniforms.light_color.rgb * specular_factor * uniforms.light_params.x;
    let ambient = uniforms.ambient_color.rgb * input.color.rgb;
    let result = ambient + diffuse + specular;
    return vec4<f32>(result, input.color.a);
}
