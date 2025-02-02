@group(0) @binding(0) var<uniform> ctx: Uniform;
@group(0) @binding(1) var<storage, read> spheres: array<Sphere>;

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    let pos = in.uv.xy - 0.5;
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

        let scatter_dir = get_scattered_direction(ray_dir, hit);
        let attenuation = hit.material.albedo;

        ray_origin = hit.position + hit.normal * 1e-4;
        ray_dir = scatter_dir;
        throughput *= attenuation;
    }

    return vec4(color, 1.0);
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
