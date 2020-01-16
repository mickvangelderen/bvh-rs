use crate::vector::*;
use crate::aabb::*;
use std::convert::TryInto;

pub type Triangle = [u32; 3];
pub type Vertex = Vector3<f32>;

#[derive(Debug)]
#[repr(C)]
pub struct Node {
    pub min: Vector3<f32>,
    pub left_or_offset: u32, // right = left + 1.
    pub max: Vector3<f32>,
    pub count: u32, // leaf when std::u32::MAX, branch otherwise.
}

#[derive(Debug)]
pub struct Tree {
    pub nodes: Vec<Node>,
    pub triangles: Vec<Triangle>,
}

impl Tree {
    pub fn new(vertices: &[Vertex], triangles: &[Triangle]) -> Self {
        let aabb = {
            let first = vertices[triangles[0][0] as usize];
            let mut aabb = AABB3::from_point(first);
            for vertex_indices in triangles.iter() {
                for &vertex_index in vertex_indices.iter() {
                    let point = vertices[vertex_index as usize];
                    aabb.include_point(point);
                }
            }
            aabb
        };

        let mut tree = Tree {
            nodes: Vec::new(),
            triangles: Vec::new(),
        };

        tree.nodes.push(Node {
            min: aabb.min,
            left_or_offset: 0,
            max: aabb.max,
            count: 0,
        });

        // NOTE: Pad for cache alignment.
        tree.nodes.push(Node {
            min: Vector3::zero(),
            left_or_offset: 0,
            max: Vector3::zero(),
            count: 0,
        });

        process(&mut tree, 0, aabb, vertices, triangles);

        fn process(
            tree: &mut Tree,
            node_index: u32,
            aabb: AABB3,
            vertices: &[Vertex],
            triangles: &[Triangle],
        ) {
            let triangle_count: u32 = triangles.len().try_into().unwrap();
            if triangle_count <= 64 {
                // Leaf node.
                let offset: u32 = tree.triangles.len().try_into().unwrap();
                tree.nodes[node_index as usize].left_or_offset = offset;
                tree.nodes[node_index as usize].count = triangle_count;
                tree.triangles.extend_from_slice(triangles);
            } else {
                // Branch node.
                let child_node_index: u32 = tree.nodes.len().try_into().unwrap();
                tree.nodes[node_index as usize].left_or_offset = child_node_index;
                tree.nodes[node_index as usize].count = std::u32::MAX;

                let (left_aabb, right_aabb) = aabb.split();

                tree.nodes.push(Node {
                    min: left_aabb.min,
                    left_or_offset: 0,
                    max: left_aabb.max,
                    count: 0,
                });
                tree.nodes.push(Node {
                    min: right_aabb.min,
                    left_or_offset: 0,
                    max: right_aabb.max,
                    count: 0,
                });

                let mut child_triangles = Vec::with_capacity(triangle_count as usize);

                for &(aabb, child_node_index) in [
                    (left_aabb, child_node_index),
                    (right_aabb, child_node_index + 1),
                ]
                .iter()
                {
                    for &triangle in triangles.iter() {
                        for &vertex_index in triangle.iter() {
                            let point = vertices[vertex_index as usize];
                            if aabb.contains_point(point) {
                                child_triangles.push(triangle);
                                break;
                            }
                        }
                    }

                    // Adjust aabb to include all vertices from its triangles.
                    // for &triangle in child_triangles.iter() {
                    //     for &vertex_index in triangle.iter() {
                    //         let point = vertices[vertex_index as usize].pos_in_obj;
                    //         aabb.include_point(point);
                    //     }
                    // }

                    process(tree, child_node_index, aabb, vertices, &child_triangles[..]);

                    child_triangles.clear();
                }
            }
        };

        tree
    }
}
