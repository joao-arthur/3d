use glam::{Mat4, Vec3};
use gltf::Node;

use super::vertex::Vertex;

pub fn append_node(
    node: Node<'_>,
    parent_transform: Mat4,
    buffers: &[gltf::buffer::Data],
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
) {
    let local_transform = Mat4::from_cols_array_2d(&node.transform().matrix());
    let world_transform = parent_transform * local_transform;
    if let Some(mesh) = node.mesh() {
        for primitive in mesh.primitives() {
            if primitive.mode() != gltf::mesh::Mode::Triangles {
                continue;
            }
            let reader = primitive.reader(|buffer| buffers.get(buffer.index()).map(|data| data.0.as_slice()));
            let Some(positions) = reader.read_positions() else {
                continue;
            };
            let base_vertex = vertices.len() as u32;
            let color = primitive.material().pbr_metallic_roughness().base_color_factor();
            for position in positions {
                let position = world_transform.transform_point3(Vec3::from_array(position));
                vertices.push(Vertex { position: position.to_array(), color });
            }
            if let Some(indices_iter) = reader.read_indices() {
                indices.extend(indices_iter.into_u32().map(|index| base_vertex + index));
            } else {
                let vertex_count = vertices.len() as u32 - base_vertex;
                indices.extend((0..vertex_count).map(|index| base_vertex + index));
            }
        }
    }
    for child in node.children() {
        append_node(child, world_transform, buffers, vertices, indices);
    }
}
