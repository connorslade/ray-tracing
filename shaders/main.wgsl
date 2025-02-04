@group(0) @binding(0) var<uniform> ctx: Uniform;
@group(0) @binding(1) var<storage, read_write> accumulation: array<vec3f>;

@group(0) @binding(2) var<storage, read> spheres: array<Sphere>;

@group(0) @binding(3) var<storage, read> models: array<Model>;
@group(0) @binding(4) var<storage, read> nodes: array<BvhNode>;
@group(0) @binding(5) var<storage, read> faces: array<Triangle>;

const PI: f32 = 3.141592653589793;

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel = vec2u(in.uv * vec2f(ctx.window));
    let pixel_idx = pixel.y * ctx.window.x + pixel.x;
    let pos = in.uv.xy - 0.5;

    seed = (pixel_idx * 2479898233) ^ (ctx.frame * 98379842);

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
    let offset = (vec2(rand(), rand()) * 2.0 - 1.0) / vec2f(ctx.window);
    let dir = ray_direction(pos + offset);
    var ray = Ray(ctx.camera.pos, dir, 1.0 / dir);

    var light = vec3(0.0);
    var color = vec3(1.0);

    for (var bounce = 0u; bounce < ctx.max_bounces; bounce++) {
        let trace = trace_ray(ray);

        if trace.hit.t < 0.0 {
            // light += background_color(ray.dir) * color * ctx.enviroment;
            light += vec3(0.1) * color * ctx.enviroment;
            break;
        }

        let emitted = trace.material.emission_color * trace.material.emission_strength;

        let scatter = get_scattered_direction(ray, trace);
        light += emitted * color;
        color *= scatter.color;

        ray = Ray(
            trace.hit.position + trace.hit.normal * 0.0001,
            scatter.direction,
            1.0 / scatter.direction
        );
    }

    return light;
}

fn trace_ray(ray: Ray) -> TraceResult {
    var hit = default_hit();
    var material = default_material();

    for (var i = 0u; i < arrayLength(&spheres); i++) {
        let sphere = spheres[i];
        let result = hit_sphere(sphere, ray);

        if result.t > 0.0 && (result.t < hit.t || hit.t < 0.0) {
            hit = result;
            material = sphere.material;
        }
    }

    var stack = array<u32, 32>();
    var pointer = 0;

    for (var i = 0u; i < arrayLength(&models); i++) {
        let model = models[i];

        let model_pos = (model.inv_transformation * vec4(ray.pos, 1.0)).xyz;
        let model_dir = (model.inv_transformation * vec4(ray.dir, 0.0)).xyz;
        let model_ray = Ray(model_pos, model_dir, 1.0 / model_dir);

        stack[0] = 0u;
        pointer = 1;

        while pointer > 0 {
            pointer--;
            let node = nodes[model.node_offset + stack[pointer]];

            if node.face_count == 0 {
                let left = nodes[model.node_offset + node.index];
                let right = nodes[model.node_offset + node.index + 1];

                let left_dist = hit_bounding_box(left.bounds, model_ray);
                let right_dist = hit_bounding_box(right.bounds, model_ray);

                if left_dist > 0.0 && (left_dist < hit.t || hit.t < 0.0) { stack[pointer] = node.index; pointer++; }
                if right_dist > 0.0 && (right_dist < hit.t || hit.t < 0.0) { stack[pointer] = node.index + 1; pointer++; }
            } else {
                for (var j = 0u; j < node.face_count; j++) {
                    let triangle = faces[model.face_offset + node.index + j];
                    let result = hit_triangle(triangle, model_ray);

                    if result.t > 0.0 && (result.t < hit.t || hit.t < 0.0) {
                        hit = result;
                        hit.normal = (model.transformation * vec4(hit.normal, 0.0)).xyz;
                        material = model.material;
                    }
                }
            }
        }
    }

    return TraceResult(hit, material);
}
