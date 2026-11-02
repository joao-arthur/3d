use glam::{Mat4, Vec3};
use wgpu::{
    BufferUsages, Device,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::data::{append_node::append_node, model::Model};

pub fn get_model(device: &Device) -> Model {
    let (document, buffers, _images) =
        gltf::import("assets/tree.glb").expect("Failed to load assets/tree.glb");
    let scene = document
        .default_scene()
        .or_else(|| document.scenes().next())
        .expect("GLB contains no scene");
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for node in scene.nodes() {
        append_node(node, Mat4::IDENTITY, &buffers, &mut vertices, &mut indices);
    }
    assert!(!vertices.is_empty(), "GLB contains no triangle vertices");
    assert!(!indices.is_empty(), "GLB contains no triangle indices");
    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);
    for vertex in &vertices {
        let position = Vec3::from_array(vertex.position);
        min = min.min(position);
        max = max.max(position);
    }
    let center = (min + max) * 0.5;
    let dimensions = max - min;
    let largest_dimension = dimensions.max_element().max(0.0001);
    let scale = 2.0 / largest_dimension;
    for vertex in &mut vertices {
        let position = Vec3::from_array(vertex.position);
        vertex.position = ((position - center) * scale).to_array();
    }
    println!("Loaded tree: {} vertices, {} indices", vertices.len(), indices.len(),);
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Tree Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Tree Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: BufferUsages::INDEX,
    });
    Model { vertex_buffer, index_buffer, index_count: indices.len() as u32 }
}
