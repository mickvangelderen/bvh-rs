use crate::aabb::*;
use crate::vector::*;
use std::convert::TryInto;

pub type Triangle = [u32; 3];

#[derive(Debug)]
#[repr(C)]
pub struct Node {
    pub min: Vector3<f32>,
    pub left_or_offset: u32, // right = left + 1.
    pub max: Vector3<f32>,
    pub count: u32, // leaf when std::u32::MAX, branch otherwise.
}

impl Node {
    pub fn unprocessed() -> Self {
        Node {
            min: Vector3::zero(),
            left_or_offset: 0,
            max: Vector3::zero(),
            count: 0,
        }
    }
}

#[derive(Debug)]
pub struct Tree {
    pub nodes: Vec<Node>,
    pub triangles: Vec<Triangle>,
}

impl Tree {
    pub fn new(vertices: &[Vector3<f32>], triangles: &[Triangle]) -> Self {
        let centroids: Vec<Vector3<f32>> = triangles
            .iter()
            .map(|&t| {
                (vertices[t[0] as usize] + vertices[t[1] as usize] + vertices[t[2] as usize]) / 3.0
            })
            .collect();

        let centroid_aabb = AABB3::from_points(centroids.iter().copied()).unwrap();
        let triangle_indices: Vec<u32> = (0u32..triangles.len().try_into().unwrap())
            .into_iter()
            .collect();

        let mut tree = Tree {
            nodes: Vec::new(),
            triangles: Vec::new(),
        };

        tree.nodes.push(Node::unprocessed());
        tree.nodes.push(Node::unprocessed());

        let triangle_aabb = process(
            &mut tree,
            &vertices,
            &triangles,
            &centroids,
            0,
            0,
            centroid_aabb,
            &triangle_indices,
        );

        tree.nodes[0].min = triangle_aabb.min;
        tree.nodes[0].max = triangle_aabb.max;

        fn process(
            tree: &mut Tree,
            vertices: &[Vector3<f32>],
            triangles: &[Triangle],
            centroids: &[Vector3<f32>],
            depth: u32,
            node_index: u32,
            centroid_aabb: AABB3,
            triangle_indices: &[u32],
        ) -> AABB3 {
            let triangle_count: u32 = triangle_indices.len().try_into().unwrap();
            let triangle_aabb = if triangle_count <= 16 || depth == 18 {
                // Leaf node.
                let offset: u32 = tree.triangles.len().try_into().unwrap();
                tree.nodes[node_index as usize].left_or_offset = offset;
                tree.nodes[node_index as usize].count = triangle_count;
                tree.triangles.extend(
                    triangle_indices
                        .iter()
                        .map(|&index| triangles[index as usize]),
                );
                AABB3::from_points(
                    tree.triangles
                        .iter()
                        .skip(offset as usize)
                        .take(triangle_count as usize)
                        .flat_map(|triangle| triangle.iter().map(|&i| vertices[i as usize])),
                )
                .unwrap()
            } else {
                let split_axis = (centroid_aabb.max - centroid_aabb.min).largest_component();
                // let split_value =
                //     (centroid_aabb.min[split_axis] + centroid_aabb.max[split_axis]) * 0.5;
                let scale = 1.0 / triangle_indices.len() as f32;
                let split_value = triangle_indices
                    .iter()
                    .map(|&i| centroids[i as usize][split_axis] * scale)
                    .sum();

                let mut left_indices = Vec::with_capacity(triangle_count as usize);
                let mut right_indices = Vec::with_capacity(triangle_count as usize);
                let mut left_centroid_aabb = AABB3::default();
                let mut right_centroid_aabb = AABB3::default();

                for &triangle_index in triangle_indices.iter() {
                    let centroid = centroids[triangle_index as usize];
                    let (centroid_aabb, indices) = if centroid[split_axis] < split_value {
                        (&mut left_centroid_aabb, &mut left_indices)
                    } else {
                        (&mut right_centroid_aabb, &mut right_indices)
                    };
                    centroid_aabb.include_point(centroid);
                    indices.push(triangle_index);
                }

                // Branch.
                let left_node_index: u32 = tree.nodes.len().try_into().unwrap();
                tree.nodes.push(Node::unprocessed());
                tree.nodes.push(Node::unprocessed());
                tree.nodes[node_index as usize].left_or_offset = left_node_index;
                tree.nodes[node_index as usize].count = std::u32::MAX;

                let left_triangle_aabb = process(
                    tree,
                    vertices,
                    triangles,
                    centroids,
                    depth + 1,
                    left_node_index,
                    left_centroid_aabb,
                    &left_indices,
                );

                let right_triangle_aabb = process(
                    tree,
                    vertices,
                    triangles,
                    centroids,
                    depth + 1,
                    left_node_index + 1,
                    right_centroid_aabb,
                    &right_indices,
                );

                AABB3::merge(left_triangle_aabb, right_triangle_aabb)
            };

            tree.nodes[node_index as usize].min = triangle_aabb.min;
            tree.nodes[node_index as usize].max = triangle_aabb.max;

            triangle_aabb
        };

        tree
    }
}
