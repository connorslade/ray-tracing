struct Uniform {
    window: vec2u,
    camera: Camera,
    frame: u32,
    accumulation_frame: u32,
    flags: u32,

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
    diffuse_color: vec3f,
    specular_color: vec3f,

    specular_probability: f32,
    roughness: f32,

    emission_color: vec3f,
    emission_strength: f32,
}

struct Model {
    material: Material,
    vertex_start: u32,
    index_start: u32,
}

struct Vertex {
    position: vec3f,
    normal: vec3f
}

struct Ray {
    pos: vec3f,
    dir: vec3f,
}

struct ScatterResult {
    direction: vec3f,
    color: vec3f
}

struct Intersection {
    hit: bool,
    material: Material,
    normal: vec3f,
    position: vec3f
}

fn intersection_miss() -> Intersection {
    return Intersection(false, default_material(), vec3f(0.0), vec3f(0.0));
}

fn default_material() -> Material {
    return Material(vec3(1.0), vec3(0.0), 0.0, 0.0, vec3(0.0), 0.0);
}
