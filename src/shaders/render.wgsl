@group(0) @binding(0) var<uniform> ctx: Uniform;

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
    let t = hit_sphere(vec3f(0, 0, -2), 0.5, ray_origin, ray_dir);

    var color = vec3(0.0); // Default: background
    if (t > 0.0) {
      let hit_point = ray_origin + t * ray_dir;
      let normal = normalize(hit_point - vec3(0, 0, -2)); // Sphere center at (0,0,-2)
      color = vec3(max(dot(normal, ctx.light_dir), 0.0));
    }

    return vec4(color, 1.0);
}

// Intersection Tests //

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
