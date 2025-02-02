struct Uniform {
    window: vec2u,
    camera: Camera,
    frame: u32,
    accumulation_frame: u32,

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

struct Sphere {
    position: vec3f,
    radius: f32,
    material: Material,
}

struct Hit {
    position: vec3f,
    normal: vec3f,
    material: Material,
    t: f32
}

fn default_hit() -> Hit {
    return Hit(
        vec3(0.0),
        vec3(0.0),
        Material(vec3(0.0), vec3(0.0), 1.0),
        -1.0
    );
}
