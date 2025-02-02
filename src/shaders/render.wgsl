@group(0) @binding(0) var<uniform> ctx: Uniform;
@group(0) @binding(1) var<storage, read> spheres: array<Sphere>;

// Type Defintions //

struct Uniform {
    camera: Camera,
    light_dir: vec3f
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
    metallic: f32,
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

// Vertex Shader //

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(1) uv: vec2<f32>
};

@vertex
fn vert(
    @location(0) pos: vec4<f32>,
    @location(1) uv: vec2<f32>,
) -> VertexOutput {
    return VertexOutput(pos, uv);
}

// Fragment Shader //

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    let pos = in.uv.xy - 0.5;
    let ray_dir = ray_direction(pos);
    let ray_origin = ctx.camera.pos;

    let hit = trace_ray(ray_origin, ray_dir);
    var color = background_color(ray_dir);

    if (hit.t > 0.0) {
        color = vec3(max(dot(hit.normal, ctx.light_dir), 0.0));
    }

    return vec4(color, 1.0);
}

// Intersection Tests //

fn trace_ray(ray_origin: vec3f, ray_dir: vec3f) -> Hit {
    var hit = default_hit();

    for (var i = 0u; i < arrayLength(&spheres); i++) {
        let sphere = spheres[i];

        let t = hit_sphere(sphere.position, sphere.radius, ray_origin, ray_dir);

        if t > 0.0 && (t < hit.t || hit.t < 0.0) {
            let position =  ray_origin + ray_dir * t;
            hit = Hit(
                position,
                normalize(position - sphere.position),
                sphere.material,
                t
            );
        }
    }

    return hit;
}

fn hit_sphere(center: vec3f, radius: f32, ray_origin: vec3f, ray_dir: vec3f) -> f32 {
    let oc = ray_origin - center;
    let a = dot(ray_dir, ray_dir);
    let b = 2.0 * dot(oc, ray_dir);
    let c = dot(oc, oc) - radius * radius;
    let disc = b * b - 4.0 * a * c;

    if disc < 0.0 { return -1.0; }
    return (-b - sqrt(disc)) / (2.0 * a);
}

// Ray Utils //

fn ray_direction(pos: vec2f) -> vec3f {
    let forward = camera_direction();
    let right = normalize(cross(vec3f(0, 1, 0), forward));
    let up = normalize(cross(forward, right));

    let fov_scale = tan(ctx.camera.fov * 0.5);
    let uv = pos * vec2(ctx.camera.aspect, 1.0) * fov_scale;

    return normalize(forward + right * uv.x + up * uv.y);
}

fn camera_direction() -> vec3f {
    var pitch = ctx.camera.pitch;
    var yaw = ctx.camera.yaw;

    return normalize(vec3(
        cos(yaw) * cos(pitch),
        sin(pitch),
        sin(yaw) * cos(pitch)
    ));
}

// Misc Functions //

fn background_color(ray_dir: vec3f) -> vec3f {
    let a = 0.5 * (ray_dir.y + 1.0);
    return (1.0 - a) * vec3(1.0, 1.0, 1.0) + a * vec3(0.5, 0.7, 1.0);
}

fn default_hit() -> Hit {
    return Hit(
        vec3(0.0),
        vec3(0.0),
        Material(vec3(0.0), vec3(0.0), 1.0, 0.0),
        -1.0
    );
}
