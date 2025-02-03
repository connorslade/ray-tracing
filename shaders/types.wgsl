struct Uniform {
    window: vec2u,
    camera: Camera,
    frame: u32,
    accumulation_frame: u32,

    enviroment: f32,
    max_bounces: u32,
    samples: u32,
}

struct Camera {
    pos: vec3f,
    pitch: f32,
    yaw: f32,

    fov: f32,
    aspect: f32,
}

struct Material {
    albedo: vec3f,
    emission: vec3f,
    roughness: f32,
}

struct Model {
    material: Material,
    node_offset: u32,
    face_offset: u32,
}

struct Sphere {
    position: vec3f,
    radius: f32,
    material: Material,
}

struct Triangle {
    v0: vec3f,
    v1: vec3f,
    v2: vec3f,

    n0: vec3f,
    n1: vec3f,
    n2: vec3f,
}

struct BvhNode {
    bounds: BoundingBox,
    index: u32,
    face_count: u32,
}

struct BoundingBox {
    min: vec3f,
    max: vec3f,
}

struct Ray {
    pos: vec3f,
    dir: vec3f
}

struct Hit {
    position: vec3f,
    normal: vec3f,
    t: f32
}

struct TraceResult {
    hit: Hit,
    material: Material
}

fn default_hit() -> Hit {
    return Hit(vec3(0.0), vec3(0.0), -1.0);
}

fn default_material() -> Material {
    return Material(vec3(1.0, 1.0, 1.0), vec3(0.0), 0.0);
}
