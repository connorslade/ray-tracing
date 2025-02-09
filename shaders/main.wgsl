@group(0) @binding(0) var<uniform> ctx: Uniform;
@group(0) @binding(1) var<storage, read_write> accumulation: array<vec3f>;
@group(0) @binding(2) var<storage, read> models: array<Model>;
@group(0) @binding(3) var acceleration: acceleration_structure;

@group(0) @binding(4) var<storage, read> vertex: array<Vertex>;
@group(0) @binding(5) var<storage, read> index: array<u32>;

@group(0) @binding(6) var texture_sampler: sampler;
@group(0) @binding(7) var textures: binding_array<texture_2d<f32>>;

const PI: f32 = 3.141592653589793;

@compute
@workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if global_id.x >= ctx.window.x || global_id.y >= ctx.window.y { return; }

    let pixel_idx = global_id.y * ctx.window.x + global_id.x;
    let uv = vec2f(global_id.xy) / vec2f(ctx.window);
    let pos = vec2f(uv.x, 1.0 - uv.y) - 0.5;

    seed = (pixel_idx * 2479898233) ^ (ctx.frame * 98379842);

    var color = vec3(0.0);
    for (var i = 0u; i < ctx.samples; i++) {
        color += sample(pos);
    }
    color /= f32(ctx.samples);

    let out = mix(accumulation[pixel_idx], color, 1.0 / f32(ctx.accumulation_frame));
    accumulation[pixel_idx] = out;
}

fn sample(pos: vec2f) -> vec3f {
    let offset = (vec2(rand(), rand()) * 2.0 - 1.0) / vec2f(ctx.window);
    let dir = ray_direction(pos + offset);
    var ray = Ray(ctx.camera.pos, dir);

    var light = vec3(0.0);
    var color = vec3(1.0);

    for (var bounce = 0u; bounce <= ctx.max_bounces; bounce++) {
        let trace = trace_ray(ray);

        if !trace.hit {
            light += background_color(ray.dir) * color * ctx.enviroment;
            // light += vec3(0.3) * color * ctx.enviroment;
            break;
        }

        // 0 => Metal; 1 => Dielectric
        if trace.material.tag == 0 {
            let material = trace.material.metal;

            let emitted = material.emission_color * material.emission_strength;
            let scatter = get_scattered_direction_metal(ray, trace, material);
            light += emitted * color;
            color *= scatter.color;

            ray = Ray(trace.position + trace.normal * 0.0001, scatter.direction);
        } else if trace.material.tag == 1 {
            let material = trace.material.dielectric;
            let next_dir = get_scattered_direction_dielectric(ray, trace, material);

            let offset_dir = trace.normal - 2.0 * trace.normal * f32(trace.front_face);
            ray = Ray(trace.position + offset_dir * 0.0001, next_dir);
        }
    }

    return light;
}

// Docs for rayQuery functions: https://github.com/gfx-rs/wgpu/blob/trunk/etc/specs/ray_tracing.md
fn trace_ray(ray: Ray) -> Intersection {
    let flags = 0x10 * (ctx.flags & 1);
    let ray_desc = RayDesc(flags, 0xFF, 0.001, 3.40282347e+38f, ray.pos, ray.dir);

    var rq: ray_query;
    rayQueryInitialize(&rq, acceleration, ray_desc);
    rayQueryProceed(&rq);

    let intersection = rayQueryGetCommittedIntersection(&rq);
    if intersection.kind == RAY_QUERY_INTERSECTION_NONE { return intersection_miss(); }

    let model = models[intersection.geometry_index];
    let index_start = model.index_start + intersection.primitive_index * 3;

    let v0 = vertex[model.vertex_start + index[index_start]];
    let v1 = vertex[model.vertex_start + index[index_start + 1]];
    let v2 = vertex[model.vertex_start + index[index_start + 2]];

    let bary = vec3f(1.0 - intersection.barycentrics.x - intersection.barycentrics.y, intersection.barycentrics);
    let normal = v0.normal * bary.x + v1.normal * bary.y + v2.normal * bary.z;
    let position = v0.position * bary.x + v1.position * bary.y + v2.position * bary.z;
    let uv = v0.uv * bary.x + v1.uv * bary.y + v2.uv * bary.z;

    let transformed_position = (intersection.object_to_world * vec4f(position, 1.0)).xyz;
    let transformed_normal = (intersection.object_to_world * vec4f(normal, 0.0)).xyz;

    return Intersection(true, intersection.front_face, model.material, transformed_normal, transformed_position, uv);
}

fn schlick_approximation(cos_theta: f32, refractive_index: f32) -> f32 {
    let r = (1.0 - refractive_index) / (1.0 + refractive_index);
    let rs = r * r;
    return rs + (1.0 - rs) * pow(1.0 - cos_theta, 5.0);
}
