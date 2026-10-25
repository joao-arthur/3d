use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

fn load_model(device: &wgpu::Device) -> Model {
    let (document, buffers, _) = gltf::import("assets/tree.glb").expect("failed to load assets/tree.glb");
    let mut vertices = Vec::<Vertex>::new();
    let mut indices = Vec::<u32>::new();
    for mesh in document.meshes() {
        for primitive in mesh.primitives() {
            let reader =primitive.reader(|buffer| {Some(&buffers[buffer.index()].0)});
            let positions =reader.read_positions().expect("mesh primitive has no positions");
            let base_vertex = vertices.len() as u32;
            for position in positions {
                let transform = Mat4::from_cols_array_2d(&node.transform().matrix());
                let p = transform.transform_point3(Vec3::new(position[0], position[1], position[2]));
                vertices.push(Vertex {position: p.to_array(),});
                vertices.push(Vertex {position});
            }
            let primitive_indices =reader.read_indices().expect("this example requires indexed geometry");
            for index in primitive_indices.into_u32() {
                indices.push(base_vertex + index);
            }
        }
    }
    println!("Loaded {} vertices, {} indices",vertices.len(),indices.len());
    let vertex_buffer = device.create_buffer_init( &wgpu::util::BufferInitDescriptor {
        label: Some("tree vertex buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("tree index buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });
    Model {
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
    }
}
