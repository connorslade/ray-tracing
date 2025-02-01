@group(0) @binding(0) var<uniform> ctx: Uniform;

struct Uniform {
    camera: Camera
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

    return vec4(ray_dir.xyz, 1.0);
}

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
