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
    var ray = Ray(ctx.camera.pos, ray_direction(pos + offset));

    var light = vec3(0.0);
    var color = vec3(1.0);

    for (var bounce = 0u; bounce <= ctx.max_bounces; bounce++) {
        let trace = trace_ray(ray);

        if trace.hit.t < 0.0 {
            light += background_color(ray.dir) * color * ctx.enviroment;
            break;
        }

        let material = trace.material;
        let emitted = material.emission * material.emission_strength;
        light += emitted * color;
        color *= material.albedo;

        ray = Ray(
            trace.hit.position,
            get_scattered_direction(ray, trace)
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

    for (var i = 0u; i < arrayLength(&models); i++) {
        let model = models[i];

        var stack = array<u32, 32>();
        var pointer = 0;

        stack[pointer] = 0u;
        pointer++;

        while pointer > 0 {
            pointer--;
            let node = nodes[model.node_offset + stack[pointer]];

            let t = hit_bounding_box(node.bounds, ray);
            if t < 0.0 { continue; }

            if node.face_count == 0 {
                stack[pointer] = node.index;
                stack[pointer + 1] = node.index + 1;
                pointer += 2;
                continue;
            }

            for (var j = 0u; j < node.face_count; j++) {
                let triangle = faces[model.face_offset + node.index + j];
                let result = hit_triangle(triangle, ray);

                if result.t > 0.0 && (result.t < hit.t || hit.t < 0.0) {
                    hit = result;
                    material = model.material;
                }
            }
        }
    }

    return TraceResult(hit, material);
}
