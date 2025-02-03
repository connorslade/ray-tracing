use core::f32;

use compute::export::nalgebra::Vector3;
use encase::ShaderType;

use crate::types::Triangle;

pub struct Bvh {
    pub faces: Vec<Triangle>,
    pub nodes: Vec<BvhNode>,
}

#[derive(ShaderType, Debug, Default)]
pub struct BvhNode {
    bounds: BoundingBox,
    index: u32,
    face_count: u32,
}

#[derive(ShaderType, Debug, Default)]
pub struct BoundingBox {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl Bvh {
    pub fn from_mesh(triangles: &[Triangle]) -> Self {
        let mut faces = Vec::new();
        let mut nodes = Vec::new();

        nodes.push(BvhNode::default());
        build_bvh(0, triangles, &mut faces, &mut nodes, 32);

        Self { faces, nodes }
    }
}

impl BoundingBox {
    pub fn from_faces(faces: &[Triangle]) -> Self {
        let (mut min, mut max) = (Vector3::repeat(f32::MAX), Vector3::repeat(f32::MIN));

        for face in faces {
            for vert in face.vertices {
                min = Vector3::new(min.x.min(vert.x), min.y.min(vert.y), min.z.min(vert.z));
                max = Vector3::new(max.x.max(vert.x), max.y.max(vert.y), max.z.max(vert.z));
            }
        }

        Self { min, max }
    }
}

fn build_bvh(
    parent: usize,
    triangles: &[Triangle],
    faces: &mut Vec<Triangle>,
    nodes: &mut Vec<BvhNode>,
    depth: u32,
) {
    let bounds = BoundingBox::from_faces(triangles);

    let left_child = nodes.len();
    let parent = &mut nodes[parent];
    if triangles.len() > 1 && depth > 0 {
        parent.bounds = bounds;
        parent.index = left_child as u32;
        parent.face_count = 0;

        let (left_tris, right_tris) = split_triangles(triangles);

        nodes.push(BvhNode::default());
        nodes.push(BvhNode::default());

        build_bvh(left_child, &left_tris, faces, nodes, depth - 1);
        build_bvh(left_child + 1, &right_tris, faces, nodes, depth - 1);
    } else {
        parent.bounds = bounds;
        parent.index = faces.len() as u32;
        parent.face_count = triangles.len() as u32;
        faces.extend_from_slice(triangles);
    }
}

fn split_triangles(triangles: &[Triangle]) -> (Vec<Triangle>, Vec<Triangle>) {
    let bounds = BoundingBox::from_faces(triangles);
    let size = bounds.max - bounds.min;

    let axis = if size.x > size.y && size.x > size.z {
        0
    } else if size.y > size.z {
        1
    } else {
        2
    };

    let mut sorted_triangles = triangles.to_vec();
    sorted_triangles.sort_by(|a, b| a.center()[axis].partial_cmp(&b.center()[axis]).unwrap());

    let mid = sorted_triangles.len() / 2;
    let left = sorted_triangles[..mid].to_vec();
    let right = sorted_triangles[mid..].to_vec();

    (left, right)
}
