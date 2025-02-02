@group(0) @binding(0) var<uniform> ctx: Uniform;
@group(0) @binding(1) var<storage, read> spheres: array<Sphere>;
@group(0) @binding(2) var<storage, read_write> accumulation: array<vec3f>;

const PI: f32 = 3.141592653589793;

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel = vec2u(in.uv * vec2f(ctx.window));
    let pixel_idx = pixel.y * ctx.window.x + pixel.x;

    seed = (pixel_idx * 2479898233) ^ (ctx.frame * 98379842);

    let pos = in.uv.xy - 0.5;

    var color = vec3(0.0);
    for (var i = 0u; i < ctx.samples; i++) {
        color += main(pos);
    }
    color /= f32(ctx.samples);

    let out = mix(accumulation[pixel_idx], color, 1.0 / f32(ctx.accumulation_frame));
    accumulation[pixel_idx] = out;

    return vec4(out, 1.0);
}

fn main(pos: vec2f) -> vec3f {
    var ray_dir = ray_direction(pos);
    var ray_origin = ctx.camera.pos;

    var color = vec3(0.0);
    var throughput = vec3(1.0);

    for (var bounce = 0u; bounce <= ctx.max_bounces; bounce++) {
        let hit = trace_ray(ray_origin, ray_dir);

        if (hit.t < 0.0) {
            color += throughput * background_color(ray_dir);
            break;
        }

        color += throughput * hit.material.emission;
        throughput *= hit.material.albedo;

        ray_origin = hit.position;
        ray_dir = get_scattered_direction(ray_dir, hit);
    }

    return color;
}

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
    let oc = center - ray_origin;
    let h = dot(ray_dir, oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = h * h - c;

    if discriminant < 0 { return -1.0; }
    return (h - sqrt(discriminant));
}
