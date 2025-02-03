use core::f32;

use compute::export::nalgebra::Vector3;
use encase::ShaderType;

use crate::types::Triangle;

pub struct Bvh {
    pub faces: Vec<Triangle>,
    pub nodes: Vec<BvhNode>,
}

#[derive(ShaderType)]
pub struct BvhNode {
    bounds: BoundingBox,

    index: u32,
    face_count: u32,
}

#[derive(ShaderType)]
pub struct BoundingBox {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl Bvh {
    pub fn from_mesh(triangles: &[Triangle]) -> Self {
        let mut faces = Vec::new();
        let mut nodes = Vec::new();

        let _root = build_bvh(triangles, &mut faces, &mut nodes, 32);

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

    fn center(&self) -> Vector3<f32> {
        (self.min + self.max) / 2.0
    }
}

fn build_bvh(
    triangles: &[Triangle],
    faces: &mut Vec<Triangle>,
    nodes: &mut Vec<BvhNode>,
    depth: u32,
) -> u32 {
    let bounds = BoundingBox::from_faces(triangles);

    if triangles.len() <= 2 || depth == 0 {
        let index = faces.len() as u32;
        let face_count = triangles.len() as u32;
        faces.extend_from_slice(triangles);
        let node = BvhNode {
            bounds,
            index,
            face_count,
        };
        nodes.push(node);
        (nodes.len() - 1) as u32
    } else {
        let (left_tris, right_tris) = split_triangles(triangles);

        let left_idx = build_bvh(&left_tris, faces, nodes, depth - 1);
        let _right_idx = build_bvh(&right_tris, faces, nodes, depth - 1);

        let node = BvhNode {
            bounds,
            index: left_idx,
            face_count: 0,
        };
        nodes.push(node);
        (nodes.len() - 1) as u32
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
