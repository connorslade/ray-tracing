struct Uniform {
    window: vec2u,
    camera: Camera,
    frame: u32,
    accumulation_frame: u32,
    flags: u32,

    exposure: f32,
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
    tag: u32,
    metal: MetalMaterial,
    dielectric: DielectricMaterial
}

struct MetalMaterial {
    diffuse_color: vec3f,
    specular_color: vec3f,

    specular_probability: f32,
    roughness: f32,

    emission_color: vec3f,
    emission_strength: f32,

    diffuse_texture: u32,
    normal_texture: u32
}

struct DielectricMaterial {
    refractive_index: f32,
}

struct Model {
    material: Material,
    vertex_start: u32,
    index_start: u32,
}

struct Vertex {
    position: vec3f,
    normal: vec3f,
    uv: vec2f
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
    front_face: bool,
    material: Material,
    normal: vec3f,
    position: vec3f,
    uv: vec2f
}

fn intersection_miss() -> Intersection {
    return Intersection(false, true, default_material(), vec3f(0.0), vec3f(0.0), vec2f(0.0));
}

fn default_material() -> Material {
    return Material(0,
        MetalMaterial(vec3(1.0), vec3(0.0), 0.0, 0.0, vec3(0.0), 0.0, 0, 0),
        DielectricMaterial(1.0)
    );
}
