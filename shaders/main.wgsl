@group(0) @binding(0) var<uniform> ctx: Uniform;
@group(0) @binding(1) var<storage, read_write> accumulation: array<vec3f>;
@group(0) @binding(2) var<storage, read> models: array<Model>;
@group(0) @binding(3) var acceleration: acceleration_structure;

@group(0) @binding(4) var<storage, read> vertex: array<Vertex>;
@group(0) @binding(5) var<storage, read> index: array<u32>;


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

    for (var bounce = 0u; bounce < ctx.max_bounces; bounce++) {
        var rq: ray_query;
        rayQueryInitialize(&rq, acceleration, RayDesc(0, 0xFFu, 0.001, 3.40282347e+38f, ray.pos, ray.dir));
        rayQueryProceed(&rq);

        let intersection = rayQueryGetCommittedIntersection(&rq);
        let model = models[intersection.geometry_index];

        let index_start = model.index_start + intersection.primitive_index * 3;

        let v0 = vertex[model.vertex_start + index[index_start]];
        let v1 = vertex[model.vertex_start + index[index_start + 1]];
        let v2 = vertex[model.vertex_start + index[index_start + 2]];

        let bary = vec3f(1.0 - intersection.barycentrics.x - intersection.barycentrics.y, intersection.barycentrics);

        let normal = v0.normal * bary.x + v1.normal * bary.y + v2.normal * bary.z;
        let position = v0.position * bary.x + v1.position * bary.y + v2.position * bary.z;

        if intersection.kind == RAY_QUERY_INTERSECTION_NONE {
            light += background_color(ray.dir) * color * ctx.enviroment;
            // light += vec3(0.3) * color * ctx.enviroment;
            break;
        }

        let emitted = model.material.emission_color * model.material.emission_strength;

        let scatter = get_scattered_direction(ray, normal, model.material);
        light += emitted * color;
        color *= scatter.color;

        ray = Ray(position + normal * 0.0001, scatter.direction);
    }

    return light;
}
